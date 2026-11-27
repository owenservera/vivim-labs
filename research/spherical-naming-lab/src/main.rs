use spherical_naming_lab::{catalogue, experiment, geometry};
use experiment::{Candidate, Hit, Index};
use geometry::{Band, Encoder, Rng};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

const USAGE: &str = "\
Spherical Naming Lab — reproducible curved-space naming exploration

USAGE:
  spherical-naming-lab validate
  spherical-naming-lab catalogue [output.tsv]
  spherical-naming-lab match LIST.tsv SEED TOP [MIN_DEG MAX_DEG KAPPA]
  spherical-naming-lab pair A.tsv B.tsv SEED COUNT TOP [MIN_DEG MAX_DEG KAPPA]
  spherical-naming-lab random SEED COUNT [MIN_DEG MAX_DEG]
  spherical-naming-lab stats LIST.tsv
  spherical-naming-lab feedback LOG.tsv PAIR_ID REVIEWER VERDICT NAME NOTES

LIST header: role<TAB>name<TAB>meaning
SEED: unsigned 64-bit integer. TOP: number of distinct concept anchors.
MIN/MAX: angular degrees in [0,180]. KAPPA: [0,1000], 0 = uniform.
match: the band filters randomized knowledge matches, not the exact nearest.
pair: the band filters same-role A/B candidate pairs. TOP applies to the
      separate knowledge matches for each member. Kappa also weights these.
random: uniform unique catalogue pairs conditional on the band.
feedback: reviewer=user|llm, verdict=keep|reject|refine, NAME may be a new name.
Outputs are TSV; input and output are UTF-8. No network or model APIs.
";

fn number<T: std::str::FromStr>(value: &str, label: &str) -> Result<T, String> {
    value.parse().map_err(|_| format!("invalid {}: {}", label, value))
}
fn band(args: &[String], weighted: bool) -> Result<Band, String> {
    if args.is_empty() { return Ok(Band::all()); }
    if args.len() != if weighted { 3 } else { 2 } {
        return Err("specify both angular limits and, for match/pair, kappa".into());
    }
    Band::new(number(&args[0], "minimum angle")?, number(&args[1], "maximum angle")?,
        if weighted { number(&args[2], "kappa")? } else { 0.0 })
}
fn read_list(path: &str) -> Result<Vec<Candidate>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {}", path, e))?;
    experiment::parse_candidates(&text).map_err(|e| format!("{}: {}", path, e))
}
fn score(x: f64) -> String { format!("{:.6}", x) }
fn row(out: &mut String, fields: Vec<String>) {
    out.push_str(&fields.join("\t"));
    out.push('\n');
}
fn hit_columns(index: &Index, h: Hit) -> Vec<String> {
    let area = &index.areas[h.index];
    vec![area.key(), area.label(), area.anchor.to_string(), score(h.degrees)]
}

fn match_command(args: &[String]) -> Result<String, String> {
    if args.len() != 3 && args.len() != 6 {
        return Err("match requires LIST SEED TOP [MIN MAX KAPPA]".into());
    }
    let candidates = read_list(&args[0])?;
    let seed: u64 = number(&args[1], "seed")?;
    let top: usize = number(&args[2], "top")?;
    let band = band(&args[3..], true)?;
    let encoder = Encoder::new()?;
    let index = Index::build(&encoder)?;
    let mut rng = Rng::new(seed);
    let mut out = String::from("seed\trole\tname\tmeaning\tnearest_id\tnearest_area\tnearest_anchor\tnearest_deg\tsampled_id\tsampled_area\tsampled_anchor\tsampled_deg\tnearest_sampled_deg\n");
    for c in &candidates {
        let ranked = index.rank(&c.vector(&encoder)?);
        let nearest = ranked[0];
        let sampled = index.draw(&ranked, top, band, &mut rng)
            .map_err(|e| format!("{}: {}", c.name, e))?;
        let mut fields = vec![seed.to_string(), c.role.clone(), c.name.clone(), c.meaning.clone()];
        fields.extend(hit_columns(&index, nearest));
        fields.extend(hit_columns(&index, sampled));
        fields.push(score(index.area_distance(nearest.index, sampled.index)));
        row(&mut out, fields);
    }
    Ok(out)
}

fn pair_command(args: &[String]) -> Result<String, String> {
    if args.len() != 5 && args.len() != 8 {
        return Err("pair requires A B SEED COUNT TOP [MIN MAX KAPPA]".into());
    }
    let a = read_list(&args[0])?;
    let b = read_list(&args[1])?;
    let seed: u64 = number(&args[2], "seed")?;
    let count: usize = number(&args[3], "count")?;
    let top: usize = number(&args[4], "top")?;
    let band = band(&args[5..], true)?;
    let encoder = Encoder::new()?;
    let av = a.iter().map(|c| c.vector(&encoder)).collect::<Result<Vec<_>, _>>()?;
    let bv = b.iter().map(|c| c.vector(&encoder)).collect::<Result<Vec<_>, _>>()?;
    let mut rng = Rng::new(seed);
    let pairs = experiment::pair_lists(&a, &av, &b, &bv, count, band, &mut rng)?;
    let index = Index::build(&encoder)?;
    // Rank once per selected candidate, not once per Cartesian-product pair.
    let selected_a: std::collections::BTreeSet<_> = pairs.iter().map(|p| p.a).collect();
    let selected_b: std::collections::BTreeSet<_> = pairs.iter().map(|p| p.b).collect();
    let ar: std::collections::BTreeMap<_, _> = selected_a.into_iter()
        .map(|i| (i, index.rank(&av[i]))).collect();
    let br: std::collections::BTreeMap<_, _> = selected_b.into_iter()
        .map(|i| (i, index.rank(&bv[i]))).collect();
    let area_band = Band::new(0.0, 180.0, band.kappa)?;
    let mut out = String::from("pair_id\tseed\trole\tname_a\tmeaning_a\tname_b\tmeaning_b\tname_distance_deg\tname_distance_0_1\tempirical_percentile\ta_nearest_id\ta_nearest_area\ta_nearest_anchor\ta_nearest_deg\ta_sample_id\ta_sample_area\ta_sample_anchor\ta_sample_deg\tb_nearest_id\tb_nearest_area\tb_nearest_anchor\tb_nearest_deg\tb_sample_id\tb_sample_area\tb_sample_anchor\tb_sample_deg\tnearest_pair_deg\tsampled_pair_deg\n");
    for p in pairs {
        let ra = &ar[&p.a];
        let rb = &br[&p.b];
        let na = ra[0];
        let nb = rb[0];
        let sa = index.draw(ra, top, area_band, &mut rng)?;
        let sb = index.draw(rb, top, area_band, &mut rng)?;
        let id = format!("{}-{}-{}", experiment::pair_id(&a[p.a], &b[p.b], seed),
            index.areas[sa.index].key(), index.areas[sb.index].key());
        let mut fields = vec![
            id, seed.to_string(), a[p.a].role.clone(), a[p.a].name.clone(), a[p.a].meaning.clone(),
            b[p.b].name.clone(), b[p.b].meaning.clone(), score(p.degrees),
            score(p.degrees / 180.0), score(p.percentile),
        ];
        fields.extend(hit_columns(&index, na));
        fields.extend(hit_columns(&index, sa));
        fields.extend(hit_columns(&index, nb));
        fields.extend(hit_columns(&index, sb));
        fields.push(score(index.area_distance(na.index, nb.index)));
        fields.push(score(index.area_distance(sa.index, sb.index)));
        row(&mut out, fields);
    }
    Ok(out)
}

fn random_command(args: &[String]) -> Result<String, String> {
    if args.len() != 2 && args.len() != 4 {
        return Err("random requires SEED COUNT [MIN MAX]".into());
    }
    let seed: u64 = number(&args[0], "seed")?;
    let count: usize = number(&args[1], "count")?;
    let band = band(&args[2..], false)?;
    let index = Index::build(&Encoder::new()?)?;
    let pairs = experiment::random_areas(&index, count, band, &mut Rng::new(seed))?;
    let mut out = String::from("seed\tarea_a\tlabel_a\tanchor_a\tarea_b\tlabel_b\tanchor_b\tdistance_deg\tdistance_0_1\n");
    for (i, j, d) in pairs {
        let (a, b) = (&index.areas[i], &index.areas[j]);
        row(&mut out, vec![seed.to_string(), a.key(), a.label(), a.anchor.into(),
            b.key(), b.label(), b.anchor.into(), score(d), score(d / 180.0)]);
    }
    Ok(out)
}

fn stats_command(path: &str) -> Result<String, String> {
    let encoder = Encoder::new()?;
    let candidates = read_list(path)?;
    let vectors = candidates.iter().map(|c| c.vector(&encoder)).collect::<Result<Vec<_>, _>>()?;
    let (r, variance) = geometry::resultant(&vectors)?;
    let (mut sum, mut n) = (0.0, 0_usize);
    for i in 0..vectors.len() {
        for j in (i + 1)..vectors.len() {
            sum += geometry::angle(&vectors[i], &vectors[j]);
            n += 1;
        }
    }
    let avg = if n > 0 { score(sum / n as f64) } else { "NA".into() };
    Ok(format!("n\tmean_resultant_length\tdirectional_variance\tmean_pairwise_deg\n{}\t{}\t{}\t{}\n",
        vectors.len(), score(r), score(variance), avg))
}

fn feedback_command(args: &[String]) -> Result<String, String> {
    if args.len() != 6 {
        return Err("feedback requires LOG PAIR_ID REVIEWER VERDICT NAME NOTES".into());
    }
    if args[1..].iter().any(|s| s.is_empty() || s.chars().any(char::is_control)) {
        return Err("feedback fields must be nonempty and contain no control characters".into());
    }
    if !matches!(args[2].as_str(), "user" | "llm") || !matches!(args[3].as_str(), "keep" | "reject" | "refine") {
        return Err("reviewer must be user|llm; verdict must be keep|reject|refine".into());
    }
    let header = "unix_seconds\tpair_id\treviewer\tverdict\tpreferred_name\tnotes";
    let existing = match fs::read_to_string(&args[0]) {
        Ok(s) => s,
        Err(e) if e.kind() == io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(e.to_string()),
    };
    if !existing.is_empty() && (existing.lines().next() != Some(header) || !existing.ends_with('\n')) {
        return Err("feedback file has incompatible header or incomplete final record".into());
    }
    let now = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| e.to_string())?.as_secs();
    let mut record = if existing.is_empty() { format!("{}\n", header) } else { String::new() };
    record.push_str(&format!("{}\t{}\n", now, args[1..].join("\t")));
    let mut file = OpenOptions::new().create(true).append(true).open(&args[0]).map_err(|e| e.to_string())?;
    file.write_all(record.as_bytes()).map_err(|e| e.to_string())?;
    file.sync_data().map_err(|e| e.to_string())?;
    Ok(format!("recorded\t{}\t{}\n", args[1], args[3]))
}

fn run(args: &[String]) -> Result<String, String> {
    let command = args.first().map(String::as_str).unwrap_or("help");
    let tail = args.get(1..).unwrap_or(&[]);
    match command {
        "help" | "--help" | "-h" => Ok(USAGE.into()),
        "validate" if tail.is_empty() => {
            catalogue::validate()?;
            let encoder = Encoder::new()?;
            let index = Index::build(&encoder)?;
            Ok(format!("status\tareas\tsemantic_channels\nok\t{}\t{}\n", index.areas.len(), encoder.tags.len()))
        }
        "catalogue" if tail.len() <= 1 => {
            let content = catalogue::render(&catalogue::build()?);
            if let Some(path) = tail.first() {
                fs::write(path, content).map_err(|e| format!("{}: {}", path, e))?;
                Ok(format!("wrote\t{}\t{}\n", path, catalogue::EXPECTED_AREAS))
            } else { Ok(content) }
        }
        "match" => match_command(tail),
        "pair" => pair_command(tail),
        "random" => random_command(tail),
        "stats" if tail.len() == 1 => stats_command(&tail[0]),
        "feedback" => feedback_command(tail),
        _ => Err(format!("unknown command or incorrect arguments: {}\n{}", command, USAGE)),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(out) => {
            if let Err(e) = io::stdout().lock().write_all(out.as_bytes()) {
                if e.kind() != io::ErrorKind::BrokenPipe {
                    eprintln!("output error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(2);
        }
    }
}
