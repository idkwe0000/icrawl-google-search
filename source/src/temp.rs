use std::{sync::OnceLock, time::SystemTime};

use anyhow::{Context, Ok, anyhow};

use crate::types::SearchRequestResponseEntry;

const ARC_DETAILS_FILENAME: &str = "arc-ua.cache";

static TEMP_PATH: OnceLock<String> = OnceLock::new();

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ArcCache {
    ua: String,
    arc: String,
}

pub fn store_search_results(data: Vec<SearchRequestResponseEntry>) -> anyhow::Result<String> {
    #[derive(Debug, serde::Serialize)]
    struct ResultsObject {
        search_results: Vec<SearchRequestResponseEntry>,
    }

    let folder = TEMP_PATH.get().expect("TEMP_PATH is not initialized");
    let filename = format!(
        "{folder}{}",
        (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis()
            / 1000) as u64
    );

    let data = serde_yaml::to_string(&ResultsObject {
        search_results: data,
    })?;

    std::fs::write(&filename, data.as_bytes())?;

    Ok(filename)
}

pub fn store_arc(ua: String, arc: String) -> anyhow::Result<()> {
    let folder = TEMP_PATH.get().expect("TEMP_PATH is not initialized");
    let filename = format!("{folder}{ARC_DETAILS_FILENAME}");
    let content = serde_json::to_string(&ArcCache { ua, arc })?;

    std::fs::write(filename, content)?;

    Ok(())
}

pub fn get_arc() -> anyhow::Result<(String, String)> {
    let folder = TEMP_PATH.get().expect("TEMP_PATH is not initialized");
    let filename = format!("{folder}{ARC_DETAILS_FILENAME}");
    let content = std::fs::read_to_string(filename)?;
    let content: ArcCache = serde_json::from_str(&content)?;

    Ok((content.ua, content.arc))
}

pub fn get_arc_timestamp() -> anyhow::Result<Option<u64>> {
    let folder = TEMP_PATH.get().expect("TEMP_PATH is not initialized");
    let filename = format!("{folder}{ARC_DETAILS_FILENAME}");

    if std::fs::exists(&filename).unwrap_or(false) {
        let metadata = std::fs::metadata(filename)?;
        let crated = metadata
            .modified()
            .unwrap()
            .duration_since(SystemTime::UNIX_EPOCH)?;

        Ok(Some((crated.as_millis() / 1000) as u64))
    } else {
        Ok(None)
    }
}

fn generate_temp_path() -> anyhow::Result<String> {
    let folder = std::env::temp_dir();
    let folder = folder
        .as_path()
        .to_str()
        .context("failed to get the temp folder")?;

    Ok(format!("{}/icrawl-google-search-cache/", folder))
}

pub fn initialize() -> anyhow::Result<()> {
    let temp_folder = generate_temp_path()?;

    std::fs::create_dir_all(&temp_folder).map_err(|e| {
        anyhow!(
            "failed to create the fodler {}, {}",
            temp_folder,
            e.to_string()
        )
    })?;

    TEMP_PATH
        .set(temp_folder)
        .expect("TEMP_PATH already initialized");

    Ok(())
}
