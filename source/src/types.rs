use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// What you want to serach
    #[arg(long)]
    pub search: String,
    /// Starting index - use the 'next_start' field from the previous search to continue a search
    #[arg(long, default_value_t = 0)]
    pub staring_index: usize,
    /// Limit how many results to fetch from Google
    #[arg(long, default_value_t = 5)]
    pub results_limit: usize,
    /// Limit to host's languages, default is en-US
    #[arg(long, default_value = "en-US")]
    pub host_lang: String,
    /// Restrict languages, default is lang_en
    #[arg(long, default_value = "lang_en")]
    pub lang_restriction: String,
}

pub struct QueryParams {
    pub arc: String,
    pub start: usize,
    pub ua: String,
    pub hl: String,
    pub lr: String,
}

#[derive(Debug, serde::Serialize)]
pub struct GoogleUrlParams {
    pub q: String,
    pub hl: String,
    pub lr: String,
    pub ie: String,
    pub oe: String,
    pub filter: u32,
    pub start: usize,
    pub asearch: String,
    #[serde(rename = "async")]
    pub __async: String,
}

#[derive(Debug, serde::Serialize)]
pub struct SearchRequestResponseEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    pub error: bool,
    pub message: String,
}

#[derive(Debug, serde::Serialize)]
pub struct SuccessResponse {
    pub error: bool,
    pub next_start: usize,
    pub results_count: usize,
    pub results_filename: String,
}
