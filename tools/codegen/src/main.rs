use anyhow::Result;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use substreams_ethereum::Abigen;
use walkdir::WalkDir;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // If specific ABI paths are provided, only process those
    // Otherwise, process all ABIs
    let abi_base = PathBuf::from("../../abi");
    let out_base = PathBuf::from("../../src");

    let entries: Vec<PathBuf> = if args.len() > 1 {
        // Process only specified files/dirs
        args[1..]
            .iter()
            .flat_map(|arg| {
                let path = PathBuf::from(arg);
                if path.is_dir() {
                    WalkDir::new(&path)
                        .into_iter()
                        .filter_map(|e| e.ok())
                        .filter(|e| {
                            e.path().is_file()
                                && e.path().extension().and_then(|ext| ext.to_str()) == Some("json")
                        })
                        .map(|e| e.path().to_path_buf())
                        .collect::<Vec<_>>()
                } else if path.is_file() {
                    vec![path]
                } else {
                    eprintln!("Warning: {} not found, skipping", arg);
                    vec![]
                }
            })
            .collect()
    } else {
        // Process all ABIs
        WalkDir::new(&abi_base)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().is_file()
                    && e.path().extension().and_then(|ext| ext.to_str()) == Some("json")
            })
            .map(|e| e.path().to_path_buf())
            .collect()
    };

    println!("Processing {} ABI files...", entries.len());

    for json_path in &entries {
        let contract_name = json_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown_contract")
            .to_lowercase();

        // Strip the abi base prefix to get relative path
        let relative_path = if let Ok(rel) = json_path.strip_prefix(&abi_base) {
            rel.to_path_buf()
        } else if let Ok(rel) = json_path.strip_prefix("../../abi") {
            rel.to_path_buf()
        } else {
            json_path.clone()
        };

        // Convert hyphens to underscores in directory components
        let sanitized: PathBuf = relative_path
            .iter()
            .map(|c| OsString::from(c.to_str().unwrap_or("").replace('-', "_")))
            .collect();
        let mut output_path = out_base.join(sanitized);
        output_path.set_file_name(format!("{}.rs", contract_name));

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        println!("  {} -> {:?}", contract_name, output_path);
        Abigen::new(&contract_name, &json_path.to_string_lossy().to_string())?
            .generate()?
            .write_to_file(output_path)?;
    }

    println!("Done!");
    Ok(())
}
