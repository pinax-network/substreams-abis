use anyhow::Result;
use substreams_ethereum::Abigen;
use walkdir::WalkDir;
use std::path::PathBuf;

fn main() -> Result<()> {
    // Recursively walk through the `./.src` directory
    for entry in WalkDir::new("./src").into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        // Only process if this is a .json file
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            // Get the contract name from the file stem
            let contract_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown_contract");

            // Build the output path by taking the same folder and changing the extension to `.rs`
            let mut output_path = PathBuf::from(path);
            output_path.set_extension("rs");

            // Generate the bindings and write to the same folder
            Abigen::new(contract_name, &path.to_string_lossy())?
                .generate()?
                .write_to_file(output_path)?;
        }
    }

    Ok(())
}
