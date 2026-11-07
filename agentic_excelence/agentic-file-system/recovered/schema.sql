PRAGMA foreign_keys=ON;
CREATE TABLE IF NOT EXISTS nodes(id TEXT PRIMARY KEY, kind TEXT NOT NULL, label TEXT NOT NULL, data_json TEXT NOT NULL DEFAULT '{}' CHECK(json_valid(data_json)));
CREATE TABLE IF NOT EXISTS edges(id INTEGER PRIMARY KEY, src TEXT NOT NULL REFERENCES nodes(id), dst TEXT NOT NULL REFERENCES nodes(id), relation TEXT NOT NULL, evidence_id TEXT, valid_from TEXT, valid_until TEXT, UNIQUE(src,dst,relation,evidence_id));
CREATE INDEX IF NOT EXISTS edges_src ON edges(src,relation);
CREATE INDEX IF NOT EXISTS edges_dst ON edges(dst,relation);
CREATE TABLE IF NOT EXISTS source_files(id TEXT PRIMARY KEY REFERENCES nodes(id), repository TEXT NOT NULL, original_path TEXT NOT NULL, expected_sha TEXT, retrieved_sha TEXT, archive_path TEXT, kind TEXT NOT NULL, state TEXT NOT NULL CHECK(state IN ('pending','exact','changed','unavailable')), UNIQUE(repository,original_path));
CREATE TABLE IF NOT EXISTS section_types(id TEXT PRIMARY KEY REFERENCES nodes(id), description TEXT);
CREATE TABLE IF NOT EXISTS section_variants(id TEXT PRIMARY KEY REFERENCES nodes(id), file_id TEXT NOT NULL REFERENCES source_files(id), section_type_id TEXT NOT NULL REFERENCES section_types(id), heading TEXT, start_line INTEGER, end_line INTEGER, flavor TEXT, evidence_level TEXT NOT NULL DEFAULT 'heading_only');
CREATE INDEX IF NOT EXISTS variant_by_type ON section_variants(section_type_id,file_id);
CREATE TABLE IF NOT EXISTS algorithms(id TEXT PRIMARY KEY REFERENCES nodes(id), version TEXT NOT NULL, input_schema TEXT NOT NULL DEFAULT '{}' CHECK(json_valid(input_schema)), output_schema TEXT NOT NULL DEFAULT '{}' CHECK(json_valid(output_schema)), owner TEXT NOT NULL, implementation_ref TEXT, verified INTEGER NOT NULL DEFAULT 0 CHECK(verified IN (0,1)));
CREATE TABLE IF NOT EXISTS algorithm_placements(id TEXT PRIMARY KEY, algorithm_id TEXT NOT NULL REFERENCES algorithms(id), target_node_id TEXT NOT NULL REFERENCES nodes(id), context_id TEXT NOT NULL, runtime_owner TEXT NOT NULL, state TEXT NOT NULL CHECK(state IN ('proposed','validated','active','expired','revoked')), created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP, expires_at TEXT, receipt_id TEXT, UNIQUE(algorithm_id,target_node_id,context_id,runtime_owner));
CREATE TABLE IF NOT EXISTS evidence(id TEXT PRIMARY KEY, origin TEXT NOT NULL, digest TEXT, observed_at TEXT, confidence REAL CHECK(confidence BETWEEN 0 AND 1));
CREATE TABLE IF NOT EXISTS scores(node_id TEXT NOT NULL REFERENCES nodes(id), dimension TEXT NOT NULL, value REAL NOT NULL, scorer_version TEXT NOT NULL, evidence_id TEXT REFERENCES evidence(id), PRIMARY KEY(node_id,dimension,scorer_version));
CREATE TABLE IF NOT EXISTS dispositions(id TEXT PRIMARY KEY, node_id TEXT NOT NULL REFERENCES nodes(id), decision TEXT NOT NULL CHECK(decision IN ('admit','defer','quarantine','merge','archive','replace')), evidence_id TEXT REFERENCES evidence(id), rationale TEXT);
-- ISOLATED graph schema, no migration of canonical HQ DB or execution authority.

-- Algorithm mobility is a versioned plan/lease timeline, never code migration.
CREATE TABLE IF NOT EXISTS placement_events (
  id TEXT PRIMARY KEY,
  placement_id TEXT NOT NULL REFERENCES algorithm_placements(id),
  event_type TEXT NOT NULL CHECK(event_type IN ('proposed','validated','activated','expired','revoked','replaced','rejected')),
  previous_target_id TEXT REFERENCES nodes(id),
  next_target_id TEXT REFERENCES nodes(id),
  event_time TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  evidence_id TEXT REFERENCES evidence(id),
  reason TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS placement_events_by_placement ON placement_events(placement_id,event_time);
CREATE TABLE IF NOT EXISTS graph_versions (
  id TEXT PRIMARY KEY,
  source_census_sha TEXT NOT NULL,
  projection_digest TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  read_only INTEGER NOT NULL DEFAULT 1 CHECK(read_only=1)
);
