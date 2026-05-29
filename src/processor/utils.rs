use std::fs;
use std::path::PathBuf;

pub fn clear_folder(folder: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if !folder.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let caminho = entry.path();

        if caminho.is_file() {
            fs::remove_file(&caminho)?;
        } else if caminho.is_dir() {
            fs::remove_dir_all(&caminho)?;
        }
    }

    Ok(())
}
