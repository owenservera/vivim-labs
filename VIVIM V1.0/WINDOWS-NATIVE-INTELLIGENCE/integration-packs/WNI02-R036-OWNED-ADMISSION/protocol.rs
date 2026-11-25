//! Versioned protocol definitions and bounded change delivery.
//! @hq:context Incremental coordination over protocol definitions.
//! @hq:invariant A failed update leaves the active generation unchanged.
//! @hq:why Deliver only affected definition changes to consumers while preserving unrelated state.
//! @hq:see protocol/v1.tsv
use std::collections::{BTreeMap, BTreeSet, VecDeque};

const MAX_DEFINITIONS: usize = 4096;
const MAX_HISTORY: usize = 64;
const MAX_REVISION_RECORDS: usize = 16384;
pub const MAX_CHECKPOINT_BYTES: usize = 64 * 1024 * 1024;

/// @hq:context One versioned language definition and its declared dependencies.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Definition {
    pub id: String,
    pub revision: u64,
    pub dependencies: BTreeSet<String>,
    pub body: String,
}

/// @hq:invariant Every dependency must exist and the definition graph must be acyclic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot {
    definitions: BTreeMap<String, Definition>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidDefinition, DuplicateId, MissingDependency, DependencyCycle,
    StaleGeneration, NonIncreasingRevision, GenerationOverflow, ResyncRequired,
    InvalidTypedDefinition(crate::meta_language::Diagnostic),
    InvalidCheckpoint, RevisionLedgerLimit, RecoveryIdentityMissing,
}

fn valid_id(s: &str) -> bool {
    !s.is_empty() && s.len() <= 128 && s.as_bytes()[0].is_ascii_lowercase()
        && s.bytes().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || b"._-".contains(&c))
}

impl Snapshot {
    /// @hq:context Validate identifiers, revision bounds and dependency closure before admitting a snapshot.
    /// @hq:why Reject cycles before they can make downstream invalidation ambiguous.
    pub fn new(definitions: Vec<Definition>) -> Result<Self, Error> {
        if definitions.len() > MAX_DEFINITIONS { return Err(Error::InvalidDefinition); }
        let mut map = BTreeMap::new();
        for d in definitions {
            if !valid_id(&d.id) || d.revision == 0 || d.dependencies.len() > 64
                || d.body.trim().is_empty() || d.body.len() > 4096
                || d.body.chars().any(char::is_control)
                || d.dependencies.iter().any(|id| !valid_id(id)) {
                return Err(Error::InvalidDefinition);
            }
            if map.insert(d.id.clone(), d).is_some() { return Err(Error::DuplicateId); }
        }
        let mut indegree = BTreeMap::new();
        let mut reverse: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for (id, d) in &map {
            indegree.insert(id.as_str(), d.dependencies.len());
            for dep in &d.dependencies {
                if !map.contains_key(dep) { return Err(Error::MissingDependency); }
                reverse.entry(dep).or_default().push(id);
            }
        }
        let mut ready: VecDeque<_> = indegree.iter().filter_map(|(id, n)| (*n == 0).then_some(*id)).collect();
        let mut seen = 0;
        while let Some(id) = ready.pop_front() {
            seen += 1;
            for dependent in reverse.get(id).into_iter().flatten() {
                let count = indegree.get_mut(dependent).expect("validated definition");
                *count -= 1;
                if *count == 0 { ready.push_back(dependent); }
            }
        }
        if seen != map.len() { return Err(Error::DependencyCycle); }
        let snapshot = Self { definitions: map };
        crate::meta_language::Program::compile(&snapshot).map_err(Error::InvalidTypedDefinition)?;
        Ok(snapshot)
    }

    /// Scaffold interchange format: id TAB revision TAB comma-separated dependencies TAB body.
    /// This is not the VLANG source grammar or the final wire format.
    /// @hq:invariant The TSV interchange format is not the source annotation grammar.
    pub fn parse(text: &str) -> Result<Self, Error> {
        if text.len() > 2 * 1024 * 1024 { return Err(Error::InvalidDefinition); }
        let mut rows = Vec::new();
        for line in text.lines().filter(|line| !line.trim().is_empty() && !line.starts_with('#')) {
            let parts: Vec<_> = line.split('\t').collect();
            if parts.len() != 4 || rows.len() == MAX_DEFINITIONS { return Err(Error::InvalidDefinition); }
            let deps: BTreeSet<String> = if parts[2].is_empty() { BTreeSet::new() }
                else { parts[2].split(',').map(str::to_owned).collect() };
            rows.push(Definition { id: parts[0].into(),
                revision: parts[1].parse().map_err(|_| Error::InvalidDefinition)?,
                dependencies: deps, body: parts[3].into() });
        }
        Self::new(rows)
    }

    pub fn get(&self, id: &str) -> Option<&Definition> { self.definitions.get(id) }
    pub fn ids(&self) -> impl Iterator<Item = &str> { self.definitions.keys().map(String::as_str) }
}

/// @hq:context Changed and removed definitions plus transitive affected consumers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Delta {
    pub base: u64,
    pub generation: u64,
    pub changed: BTreeSet<String>,
    pub removed: BTreeSet<String>,
    pub affected: BTreeSet<String>,
    /// Only new/replaced definitions, not the entire active snapshot.
    pub upserts: Vec<Definition>,
}

/// @hq:context In-memory active generation, revision history and bounded change delivery.
/// @hq:invariant Removed definitions retain their last revision in this runtime.
#[derive(Clone)]
pub struct Runtime {
    active: Snapshot,
    generation: u64,
    last_revision: BTreeMap<String, u64>,
    history: VecDeque<Delta>,
}

impl Runtime {
    pub fn start(active: Snapshot) -> Self {
        let last_revision = active.definitions.iter().map(|(id,d)| (id.clone(), d.revision)).collect();
        Self { active, generation: 1, last_revision, history: VecDeque::new() }
    }
    pub fn generation(&self) -> u64 { self.generation }
    pub fn snapshot(&self) -> &Snapshot { &self.active }

    /// @hq:context Export exact accepted definitions, generation and revision tombstones in canonical order.
    /// @hq:invariant A checkpoint is data, not authentication, policy, external state or authority.
    pub fn checkpoint(&self) -> Result<String, Error> {
        let mut out = format!("vivim.meta.checkpoint/1\t{}\n", self.generation);
        for (id, d) in &self.active.definitions {
            let deps = d.dependencies.iter().cloned().collect::<Vec<_>>().join(",");
            out.push_str(&format!("D\t{id}\t{}\t{deps}\t{}\n", d.revision, d.body));
        }
        for (id, revision) in &self.last_revision {
            out.push_str(&format!("R\t{id}\t{revision}\n"));
        }
        out.push_str(&format!("END\t{}\t{}\n", self.active.definitions.len(), self.last_revision.len()));
        if out.len() > MAX_CHECKPOINT_BYTES { return Err(Error::InvalidCheckpoint); }
        Ok(out)
    }

    /// @hq:context Reconstruct the accepted runtime from an explicitly selected checkpoint at startup.
    /// @hq:invariant The caller owns authenticated checkpoint selection and durable persistence.
    /// @hq:invariant Delta history is not persisted; old cursors must explicitly resynchronize.
    pub fn from_checkpoint(text: &str) -> Result<Self, Error> {
        if text.len() > MAX_CHECKPOINT_BYTES { return Err(Error::InvalidCheckpoint); }
        let mut lines = text.lines();
        let generation: u64 = lines.next().and_then(|s| s.strip_prefix("vivim.meta.checkpoint/1\t"))
            .and_then(|s| s.parse().ok()).filter(|n| *n > 0).ok_or(Error::InvalidCheckpoint)?;
        let mut definitions = Vec::new();
        let mut last_revision = BTreeMap::new();
        let mut ended = false;
        for line in lines {
            if ended { return Err(Error::InvalidCheckpoint); }
            let fields = line.split('\t').collect::<Vec<_>>();
            match fields.as_slice() {
                ["D", id, revision, deps, body] => {
                    if definitions.len() >= MAX_DEFINITIONS { return Err(Error::InvalidCheckpoint); }
                    definitions.push(Definition { id: (*id).into(), revision: revision.parse().map_err(|_| Error::InvalidCheckpoint)?,
                        dependencies: if deps.is_empty() { BTreeSet::new() } else { deps.split(',').map(str::to_owned).collect() }, body: (*body).into() });
                }
                ["R", id, rev] => {
                    if last_revision.len() >= MAX_REVISION_RECORDS || !valid_id(id) { return Err(Error::InvalidCheckpoint); }
                    let rev: u64 = rev.parse().map_err(|_| Error::InvalidCheckpoint)?;
                    if rev == 0 || last_revision.insert((*id).to_owned(), rev).is_some() { return Err(Error::InvalidCheckpoint); }
                }
                ["END", defs, revs] => {
                    if defs.parse::<usize>().ok() != Some(definitions.len()) || revs.parse::<usize>().ok() != Some(last_revision.len()) {
                        return Err(Error::InvalidCheckpoint);
                    }
                    ended = true;
                }
                _ => return Err(Error::InvalidCheckpoint),
            }
        }
        if !ended { return Err(Error::InvalidCheckpoint); }
        let active = Snapshot::new(definitions)?;
        if active.definitions.iter().any(|(id,d)| last_revision.get(id) != Some(&d.revision)) {
            return Err(Error::InvalidCheckpoint);
        }
        let runtime = Self { active, generation, last_revision, history: VecDeque::new() };
        // Reject truncated, noncanonical, duplicate or ambiguously encoded records.
        if runtime.checkpoint()? != text { return Err(Error::InvalidCheckpoint); }
        Ok(runtime)
    }

    /// @hq:context Generate a forward-only recovery proposal for explicitly selected identities.
    /// @hq:invariant Restore behavior as new revisions; never rewind generation or erase unrelated changes.
    pub fn recovery_patch(&self, expected: u64, historical: &Self, ids: &BTreeSet<String>) -> Result<Vec<Definition>, Error> {
        if expected != self.generation { return Err(Error::StaleGeneration); }
        if ids.is_empty() || ids.len() > MAX_DEFINITIONS { return Err(Error::InvalidDefinition); }
        let mut result = Vec::new();
        for id in ids {
            let old = historical.snapshot().get(id).ok_or(Error::RecoveryIdentityMissing)?;
            if self.snapshot().get(id).is_some_and(|current| current.body == old.body && current.dependencies == old.dependencies) {
                continue;
            }
            let mut replacement = old.clone();
            replacement.revision = self.last_revision.get(id).copied().unwrap_or(0).max(old.revision)
                .checked_add(1).ok_or(Error::GenerationOverflow)?;
            result.push(replacement);
        }
        Ok(result)
    }

    /// @hq:context Preview a protocol upgrade without activating it.
    /// @hq:invariant Preview uses exactly the same validation as replace.
    pub fn preview_replace(&self, expected: u64, candidate: Snapshot) -> Result<Option<Delta>, Error> {
        self.clone().replace(expected, candidate)
    }

    /// In-memory CAS update. Caller must authorize and persist the actual source transaction.
    /// @hq:invariant Reject stale generations and non-increasing revisions before committing state.
    /// @hq:why The union of old and new dependency graphs preserves invalidation after removal.
    pub fn replace(&mut self, expected: u64, candidate: Snapshot) -> Result<Option<Delta>, Error> {
        if expected != self.generation { return Err(Error::StaleGeneration); }
        let old_typed = crate::meta_language::Program::compile(&self.active).map_err(Error::InvalidTypedDefinition)?;
        let new_typed = crate::meta_language::Program::compile(&candidate).map_err(Error::InvalidTypedDefinition)?;
        old_typed.compatible_with(&new_typed).map_err(Error::InvalidTypedDefinition)?;
        let mut changed = BTreeSet::new();
        let mut removed = BTreeSet::new();
        for (id, d) in &candidate.definitions {
            if self.active.get(id) != Some(d) {
                if self.last_revision.get(id).is_some_and(|old| d.revision <= *old) {
                    return Err(Error::NonIncreasingRevision);
                }
                changed.insert(id.clone());
            }
        }
        for id in self.active.definitions.keys() {
            if candidate.get(id).is_none() { removed.insert(id.clone()); }
        }
        if changed.is_empty() && removed.is_empty() { return Ok(None); }
        let new_ids = changed.iter().filter(|id| !self.last_revision.contains_key(*id)).count();
        if self.last_revision.len() + new_ids > MAX_REVISION_RECORDS { return Err(Error::RevisionLedgerLimit); }
        let next = self.generation.checked_add(1).ok_or(Error::GenerationOverflow)?;
        // Union both graphs so a removed dependency also invalidates old consumers.
        let mut reverse: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
        for snapshot in [&self.active, &candidate] {
            for (id, d) in &snapshot.definitions {
                for dep in &d.dependencies { reverse.entry(dep).or_default().insert(id); }
            }
        }
        let mut affected: BTreeSet<String> = changed.union(&removed).cloned().collect();
        let mut queue: VecDeque<String> = affected.iter().cloned().collect();
        // @hq:invariant Each affected definition is enqueued only when first inserted.
        while let Some(id) = queue.pop_front() {
            if let Some(children) = reverse.get(id.as_str()) {
                for child in children {
                    if affected.insert((*child).to_owned()) { queue.push_back((*child).to_owned()); }
                }
            }
        }
        let upserts = changed.iter().map(|id| candidate.get(id).expect("changed definition").clone()).collect();
        let delta = Delta { base: self.generation, generation: next, changed, removed, affected, upserts };
        for d in &delta.upserts { self.last_revision.insert(d.id.clone(), d.revision); }
        self.active = candidate;
        self.generation = next;
        self.history.push_back(delta.clone());
        if self.history.len() > MAX_HISTORY { self.history.pop_front(); }
        Ok(Some(delta))
    }

    /// Apply only the definitions that an editor changed. Validates the candidate
    /// protocol graph before activation; no downstream full-snapshot refresh.
    /// As with replace(), caller owns persistence and authorization.
    /// @hq:context Apply bounded upserts and removals through the same snapshot validator.
    pub fn patch(&mut self, expected: u64, upserts: Vec<Definition>, removals: BTreeSet<String>) -> Result<Option<Delta>, Error> {
        if expected != self.generation { return Err(Error::StaleGeneration); }
        if upserts.len() > MAX_DEFINITIONS || removals.len() > MAX_DEFINITIONS {
            return Err(Error::InvalidDefinition);
        }
        let mut candidate = self.active.definitions.clone();
        for id in &removals {
            if candidate.remove(id).is_none() { return Err(Error::InvalidDefinition); }
        }
        let mut seen = BTreeSet::new();
        for d in upserts {
            if removals.contains(&d.id) { return Err(Error::InvalidDefinition); }
            if !seen.insert(d.id.clone()) { return Err(Error::DuplicateId); }
            candidate.insert(d.id.clone(), d);
        }
        self.replace(expected, Snapshot::new(candidate.into_values().collect())?)
    }

    /// A client that already has a snapshot requests only subsequent deltas.
    /// A missing cursor must resync explicitly; it cannot silently accept stale knowledge.
    /// @hq:invariant A cursor outside retained history requires explicit resynchronization.
    pub fn changes_since(&self, cursor: u64) -> Result<Vec<&Delta>, Error> {
        if cursor > self.generation || cursor == 0 { return Err(Error::ResyncRequired); }
        if cursor == self.generation { return Ok(Vec::new()); }
        if self.history.front().map_or(true, |first| cursor < first.base) {
            return Err(Error::ResyncRequired);
        }
        Ok(self.history.iter().filter(|d| d.generation > cursor).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn s(text: &str) -> Snapshot { Snapshot::parse(text).unwrap() }
    #[test]
    fn only_transitive_consumers_are_affected() {
        let mut r = Runtime::start(s("tag\t1\t\told\ncontext\t1\ttag\tctx\nwiki\t1\tcontext\tpage\nother\t1\t\tunrelated"));
        let d = r.replace(1, s("tag\t2\t\tnew\ncontext\t1\ttag\tctx\nwiki\t1\tcontext\tpage\nother\t1\t\tunrelated")).unwrap().unwrap();
        assert_eq!(d.affected, ["context","tag","wiki"].into_iter().map(str::to_owned).collect());
        assert_eq!(d.upserts.len(), 1);
        assert_eq!(r.changes_since(1).unwrap(), vec![&d]);
        assert!(r.changes_since(2).unwrap().is_empty());
    }
    #[test]
    fn no_op_stale_and_version_checks() {
        let first = s("tag\t1\t\told");
        let mut r = Runtime::start(first.clone());
        assert_eq!(r.replace(1, first), Ok(None));
        assert_eq!(r.replace(0, s("tag\t2\t\tnew")), Err(Error::StaleGeneration));
        assert_eq!(r.replace(1, s("tag\t1\t\tnew")), Err(Error::NonIncreasingRevision));
        assert_eq!(r.generation(), 1);
    }
    #[test]
    fn removals_and_old_dependencies() {
        let mut r = Runtime::start(s("tag\t1\t\told\ncontext\t1\ttag\tctx\nwiki\t1\tcontext\tpage"));
        let d = r.replace(1, s("context\t2\t\tctx\nwiki\t1\tcontext\tpage")).unwrap().unwrap();
        assert!(d.removed.contains("tag") && d.affected.contains("wiki"));
        assert_eq!(r.replace(2, s("tag\t1\t\told\ncontext\t2\t\tctx\nwiki\t1\tcontext\tpage")), Err(Error::NonIncreasingRevision));
    }
    #[test]
    fn rejects_invalid_graph() {
        assert_eq!(Snapshot::parse("a\t1\tb\tx"), Err(Error::MissingDependency));
        assert_eq!(Snapshot::parse("a\t1\tb\tx\nb\t1\ta\ty"), Err(Error::DependencyCycle));
        assert_eq!(Snapshot::parse("a\t1\t\tx\na\t2\t\ty"), Err(Error::DuplicateId));
    }
    #[test]
    fn direct_definition_patch_preserves_unrelated_state() {
        let mut r = Runtime::start(s("tag\t1\t\told\ncontext\t1\ttag\tctx\nother\t1\t\tunrelated"));
        let mut updated = r.snapshot().get("tag").unwrap().clone();
        updated.revision = 2;
        updated.body = "new".into();
        let d = r.patch(1, vec![updated.clone()], BTreeSet::new()).unwrap().unwrap();
        assert_eq!(d.affected, ["context", "tag"].into_iter().map(str::to_owned).collect());
        assert_eq!(d.upserts, vec![updated.clone()]);
        assert_eq!(r.snapshot().get("other").unwrap().body, "unrelated");
        assert_eq!(r.patch(1, vec![updated.clone()], BTreeSet::new()), Err(Error::StaleGeneration));
        assert_eq!(r.patch(2, vec![updated.clone(), updated], BTreeSet::new()), Err(Error::DuplicateId));
        assert_eq!(r.patch(2, vec![], ["tag".into()].into_iter().collect()), Err(Error::MissingDependency));
        assert_eq!(r.generation(), 2);
    }
    #[test]
    fn preview_matches_actual_change_and_never_mutates_runtime() {
        let mut r = Runtime::start(s("tag\t1\t\told\ncontext\t1\ttag\tctx\nother\t1\t\tunrelated"));
        let candidate = s("tag\t2\t\tnew\ncontext\t1\ttag\tctx\nother\t1\t\tunrelated");
        let before = r.snapshot().clone();
        let preview = r.preview_replace(1, candidate.clone()).unwrap().unwrap();
        assert_eq!(r.generation(), 1);
        assert_eq!(r.snapshot(), &before);
        assert!(r.changes_since(1).unwrap().is_empty());
        assert_eq!(preview.affected, ["tag","context"].into_iter().map(str::to_owned).collect());
        assert_eq!(r.replace(1, candidate).unwrap(), Some(preview));
        assert_eq!(r.preview_replace(1, before), Err(Error::StaleGeneration));
        assert_eq!(r.preview_replace(2,s("tag\t2\t\tdifferent")),Err(Error::NonIncreasingRevision));
    }
    #[test]
    fn history_gap_requires_resync() {
        let mut r = Runtime::start(s("tag\t1\t\tx"));
        for version in 2..=67 {
            r.replace(version - 1, s(&format!("tag\t{version}\t\tx"))).unwrap();
        }
        assert_eq!(r.changes_since(1), Err(Error::ResyncRequired));
        assert_eq!(r.changes_since(66).unwrap().len(), 1);
        assert_eq!(r.changes_since(68), Err(Error::ResyncRequired));
    }

    #[test]
    fn checkpoint_preserves_revision_tombstones_and_requires_resync(){
        let mut r=Runtime::start(s("a\t1\t\told\nb\t3\t\tgone"));
        r.replace(1,s("a\t2\t\tnew")).unwrap();
        let encoded=r.checkpoint().unwrap();
        let mut restored=Runtime::from_checkpoint(&encoded).unwrap();
        assert_eq!(restored.generation(),2);assert_eq!(restored.snapshot(),r.snapshot());
        assert_eq!(restored.checkpoint().unwrap(),encoded);
        assert_eq!(restored.changes_since(1),Err(Error::ResyncRequired));
        assert!(restored.changes_since(2).unwrap().is_empty());
        assert_eq!(restored.replace(2,s("a\t2\t\tnew\nb\t3\t\tgone")),Err(Error::NonIncreasingRevision));
        assert!(restored.replace(2,s("a\t2\t\tnew\nb\t4\t\tback")).unwrap().is_some());
        assert!(Runtime::from_checkpoint(encoded.trim_end()).is_err());
        assert!(Runtime::from_checkpoint(&encoded.replace("R\ta\t2","R\ta\t1")).is_err());
        assert!(Runtime::from_checkpoint(&encoded.replace("END\t1\t2","END\t1\t1")).is_err());
    }
    #[test]
    fn forward_recovery_preserves_unrelated_edits_and_revision_monotonicity(){
        let mut r=Runtime::start(s("a\t1\t\told\nb\t1\ta\tconsumer"));
        let historical=Runtime::from_checkpoint(&r.checkpoint().unwrap()).unwrap();
        r.replace(1,s("a\t2\t\tnew\nb\t1\ta\tconsumer\nc\t1\t\tunrelated")).unwrap();
        let ids=["a".to_owned()].into_iter().collect();
        let patch=r.recovery_patch(2,&historical,&ids).unwrap();
        assert_eq!(patch[0].revision,3);assert_eq!(patch[0].body,"old");
        let d=r.patch(2,patch,BTreeSet::new()).unwrap().unwrap();
        assert_eq!(d.generation,3);assert_eq!(d.affected,["a","b"].into_iter().map(str::to_owned).collect());
        assert_eq!(r.snapshot().get("c").unwrap().body,"unrelated");
        assert!(r.recovery_patch(3,&historical,&ids).unwrap().is_empty());
        assert_eq!(r.recovery_patch(2,&historical,&ids),Err(Error::StaleGeneration));
    }
}
