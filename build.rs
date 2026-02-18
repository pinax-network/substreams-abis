use anyhow::Result;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use substreams_ethereum::Abigen;
use walkdir::WalkDir;

fn main() -> Result<()> {
    // Base directory for your generated files
    let out_base = PathBuf::from("./src");

    // Recursively walk through `./abi`
    for entry in WalkDir::new("./abi").into_iter().filter_map(|e| e.ok()) {
        let json_path = entry.path();

        // Only process `.json` files
        if json_path.is_file() && json_path.extension().and_then(|ext| ext.to_str()) == Some("json")
        {
            // Convert the file stem to lowercase
            let contract_name = json_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown_contract")
                .to_lowercase();

            // Strip off the leading `./abi/` portion so we can reconstruct a parallel folder tree
            let relative_path = json_path.strip_prefix("./abi")?;
            // Convert hyphens to underscores in directory components
            // (e.g. abi/erc20-tokens/DAI.json -> src/erc20_tokens/dai.rs)
            let sanitized: PathBuf = relative_path
                .iter()
                .map(|c| OsString::from(c.to_str().unwrap_or("").replace('-', "_")))
                .collect();
            let mut output_path = out_base.join(sanitized);
            output_path.set_file_name(format!("{}.rs", contract_name));

            // Ensure any subdirectories are created
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Generate & write
            Abigen::new(&contract_name, &json_path.to_string_lossy().to_string())?
                .generate()?
                .write_to_file(output_path)?;
        }
    }

    // Auto-format generated files (best-effort; won't fail the build if rustfmt is missing)
    let _ = std::process::Command::new("rustfmt")
        .args(["--edition", "2021"])
        .args(
            WalkDir::new("./src")
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path().is_file()
                        && e.path().extension().and_then(|ext| ext.to_str()) == Some("rs")
                })
                .map(|e| e.path().to_string_lossy().to_string())
                .collect::<Vec<_>>(),
        )
        .status();

    Ok(())
}
