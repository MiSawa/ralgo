#[macro_use]
extern crate clap;

use std::collections::{HashMap, HashSet};
use std::option::Option::Some;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::Context;
use clap::Arg;
use rls_analysis::lowering::lower_span;
use rls_analysis::{
    read_analysis_from_files, AnalysisHost, AnalysisLoader, CargoAnalysisLoader, Crate, Id, Target,
};
use rls_data::RefKind;

fn main() -> anyhow::Result<()> {
    let app = app_from_crate!()
        .arg(
            Arg::with_name("target")
                .long("target")
                .takes_value(true)
                .help("Target binary to analyze dependency")
                .required(true),
        )
        .arg(
            Arg::with_name("extract")
                .long("extract")
                .takes_value(true)
                .multiple(true)
                .help("Packages to extract")
                .default_value("ralgo"),
        );
    let matches = app.get_matches();
    let crate_to_check = matches.value_of("target").unwrap();
    let crate_to_extract = matches.value_of("extract").unwrap();
    let base_dir = Path::new(".");

    save_analysis(crate_to_check)?;
    let crates = load_analysis()?;
    let analysis_host = load_analysis_host()?;

    let mut root_ids_to_extract: HashSet<Id> = HashSet::new();
    let mut files_to_extract: HashSet<PathBuf> = HashSet::new();
    let mut files_to_check: HashSet<PathBuf> = HashSet::new();
    let mut defs: HashMap<Id, PathBuf> = HashMap::new();
    let mut uses: HashMap<PathBuf, HashSet<Id>> = HashMap::new();

    for (id, name) in analysis_host.def_roots()? {
        if name == crate_to_extract {
            root_ids_to_extract.insert(id);
        }
    }
    for krate in crates.iter() {
        let mut use_spans = Vec::new();
        use_spans.extend(
            krate
                .analysis
                .relations
                .iter()
                .map(|relation| relation.span.clone()),
        );
        use_spans.extend(krate.analysis.impls.iter().map(|imp| imp.span.clone()));
        use_spans.extend(krate.analysis.macro_refs.iter().map(|mcr| mcr.span.clone()));
        use_spans.extend(
            krate
                .analysis
                .refs
                .iter()
                .filter(|rf| rf.kind != RefKind::Mod)
                .map(|rf| rf.span.clone()),
        );
        for use_span in use_spans {
            let lowered = lower_span(&use_span, base_dir, &krate.path_rewrite);
            if krate.id.name == crate_to_check {
                // can be inside of a macro and therefore in a different file.
                files_to_check.insert(lowered.file.clone());
            }
            analysis_host
                .id(&lowered)
                .and_then(|use_id| analysis_host.get_def(use_id))
                .and_then(|def| analysis_host.id(&def.span))
                .map(|def_id| {
                    uses.entry(lowered.file)
                        .or_insert_with(|| HashSet::new())
                        .insert(def_id)
                })
                .ok();
        }

        let def_spans = krate.analysis.defs.iter().map(|def| def.span.clone());
        for def_span in def_spans {
            let lowered = lower_span(&def_span, base_dir, &krate.path_rewrite);
            analysis_host
                .id(&lowered)
                .map(|id| defs.insert(id, def_span.file_name))
                .ok();
        }
    }
    analysis_host.with_analysis(|a| {
        for (_, ca) in a.per_crate.iter() {
            for (path, _) in ca.defs_per_file.iter() {
                if ca
                    .root_id
                    .filter(|id| root_ids_to_extract.contains(id))
                    .is_some()
                {
                    files_to_extract.insert(path.clone());
                }
            }
        }
        Some(())
    })?;
    let mut pending_check: Vec<PathBuf> = files_to_check.iter().cloned().collect();
    while let Some(path) = pending_check.pop() {
        if files_to_extract.contains(&path) {
            println!("{}", path.to_string_lossy());
        }
        for id in uses.get(&path).unwrap_or(&HashSet::new()).iter() {
            analysis_host
                .get_def(*id)
                .map(|d| d.span.file)
                .map(|p| {
                    if files_to_check.insert(p.clone()) {
                        pending_check.push(p.clone());
                    }
                })
                .ok();
        }
    }
    Ok(())
}

fn load_analysis() -> anyhow::Result<Vec<Crate>> {
    let mut loader = CargoAnalysisLoader::new(Target::Debug);
    loader.set_path_prefix(Path::new("."));
    let crates = read_analysis_from_files(&loader, HashMap::new(), &[] as &[&str]);
    Ok(crates)
}

fn load_analysis_host() -> anyhow::Result<AnalysisHost> {
    let analysis_host = AnalysisHost::new(Target::Debug);
    analysis_host
        .reload(Path::new("."), Path::new("."))
        .with_context(|| "Failed to load analysis")?;
    Ok(analysis_host)
}

fn save_analysis(crate_to_check: &str) -> anyhow::Result<()> {
    Command::new("cargo")
        .env("CARGO_TARGET_DIR", "target/rls")
        .env("RUSTC_BOOTSTRAP", "1")
        .env("RUSTFLAGS", "-Z save-analysis")
        .arg("+nightly")
        .arg("check")
        .arg("-q")
        .arg("--workspace")
        .arg("--bin")
        .arg(crate_to_check)
        .status()
        .with_context(|| "Failed to build analyse file")?;
    Ok(())
}
