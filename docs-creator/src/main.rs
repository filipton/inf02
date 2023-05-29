use std::path::PathBuf;

use serde::Serialize;

#[derive(Debug, Serialize)]
struct DocsEntry {
    url: String,
    markdown: String,
}

const DOCS_GITHUB_DIR: &str = "../docs";
const FRONTEND_STATIC_DIR: &str = "../frontend/static";

fn main() -> anyhow::Result<()> {
    let mut entries: Vec<DocsEntry> = Vec::new();
    read_reccursive(&PathBuf::from(DOCS_GITHUB_DIR), &mut entries);

    let json = serde_json::to_string_pretty(&entries).unwrap();
    std::fs::write(
        format!("{}/docs.json", FRONTEND_STATIC_DIR),
        json.as_bytes(),
    )?;

    Ok(())
}

fn read_reccursive(path: &PathBuf, entries: &mut Vec<DocsEntry>) {
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".md") {
            let url = path
                .strip_prefix("../docs/")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            let markdown = std::fs::read_to_string(path).unwrap();
            entries.push(DocsEntry { url, markdown });
        } else if path.is_dir() {
            read_reccursive(&path, entries);
        }
    }
}
