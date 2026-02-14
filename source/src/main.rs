use clap::Parser;
use regex::Regex;
use reqwest::header;
use scraper::{Html, Selector};

use crate::types::{
    Args, ErrorResponse, GoogleUrlParams, QueryParams, SearchRequestResponseEntry, SuccessResponse,
};

pub mod helper;
pub mod temp;
pub mod types;

async fn query(search: String, param: QueryParams) -> anyhow::Result<String> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://www.google.com/search")
        .query(&GoogleUrlParams {
            q: search,
            hl: param.hl,
            lr: param.lr,
            ie: "utf-8".to_owned(),
            oe: "utf-8".to_owned(),
            filter: 0,
            start: param.start,
            asearch: "arc".to_owned(),
            __async: param.arc,
        })
        .header(header::USER_AGENT, param.ua)
        .header(header::ACCEPT, "*/*")
        .header(header::COOKIE, "CONSENT=YES+")
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("google returned non success code");
    }

    let response_url = response.url().to_string();

    if response_url.contains("sorry.google.com") || response_url.contains("/sorry") {
        anyhow::bail!("google showed a captcha, can't continue");
    }

    let html = response.text().await?;

    // remove the XSSI
    let regex_root_html = Regex::new("(?i)<(!DOCTYPE|html|head|body|div)").unwrap();

    if let Some(matched) = regex_root_html.find(&html) {
        let start_index = matched.start();

        Ok(html[start_index..].to_owned())
    } else {
        Ok(html)
    }
}

fn parse_search_result(html: &str) -> Option<SearchRequestResponseEntry> {
    let fragment = Html::parse_fragment(html);
    let mut result = SearchRequestResponseEntry {
        link: None,
        image: None,
        base_url: None,
        title: None,
        description: None,
        lang: None,
    };

    // find link from anchor tags with role="presentation"
    let a_selector = Selector::parse("a[role=\"presentation\"]").unwrap();
    for element in fragment.select(&a_selector) {
        if let Some(link) = element.value().attr("href") {
            if link.starts_with("/url?q=") {
                // clear the tracking link
                result.link = link.split("/url?q=").nth(1).map(|s| s.to_string());
            } else {
                result.link = Some(link.to_string());
            }
            break;
        }
    }

    // search for image
    let img_selector = Selector::parse("img").unwrap();
    for element in fragment.select(&img_selector) {
        if let Some(src) = element.value().attr("src") {
            result.image = Some(src.to_string());
            break;
        }
    }

    // parse data-snf divs - collect all divs with data-snf attribute
    let data_snf_selector = Selector::parse("div[data-snf]").unwrap();
    let data_snf_divs: Vec<_> = fragment.select(&data_snf_selector).collect();

    // parse the harder parts by index
    for (index, element) in data_snf_divs.iter().enumerate() {
        let text = element.text().collect::<String>().trim().to_string();

        match index {
            5 => result.base_url = if !text.is_empty() { Some(text) } else { None },
            7 => result.title = if !text.is_empty() { Some(text) } else { None },
            8 => result.description = if !text.is_empty() { Some(text) } else { None },
            _ => {}
        }
    }

    // skip empty results
    if result.link.is_none()
        && result.image.is_none()
        && result.base_url.is_none()
        && result.title.is_none()
        && result.description.is_none()
    {
        None
    } else {
        Some(result)
    }
}

fn parse_page(page: String) -> anyhow::Result<Vec<SearchRequestResponseEntry>> {
    let document = Html::parse_document(&page);
    let mut results = Vec::new();

    // Find all divs with data-dsrp attribute
    let data_dsrp_selector = Selector::parse("div[data-dsrp]").unwrap();

    for element in document.select(&data_dsrp_selector) {
        let html = element.html();
        if let Some(mut search_result) = parse_search_result(&html) {
            // Set lang from the parent div
            search_result.lang = element.value().attr("lang").map(|s| s.to_string());
            results.push(search_result);
        }
    }

    Ok(results)
}

async fn fetch_search_results_recursive(
    args: &Args,
    start: usize,
    mut accumulated_results: Vec<SearchRequestResponseEntry>,
) -> anyhow::Result<Vec<SearchRequestResponseEntry>> {
    // Base case: we've reached the results limit
    if accumulated_results.len() >= args.results_limit {
        return Ok(accumulated_results);
    }

    if !accumulated_results.is_empty() {
        tokio::time::sleep(tokio::time::Duration::from_secs(fastrand::u64(1..3))).await;
    }

    let helper_params = helper::get_params(start)?;
    let query_params = QueryParams {
        arc: helper_params.arc,
        start,
        ua: helper_params.ua,
        hl: args.host_lang.clone(),
        lr: args.lang_restriction.clone(),
    };

    let page = query(args.search.clone(), query_params).await?;
    let new_results = parse_page(page)?;
    let new_results_len = new_results.len();
    accumulated_results.extend(new_results);

    // Calculate new start: previous start + results from this page + 1
    let new_start = start + new_results_len + 1;

    Box::pin(fetch_search_results_recursive(
        args,
        new_start,
        accumulated_results,
    ))
    .await
}

async fn tool(args: Args) -> anyhow::Result<()> {
    temp::initialize()?;

    let search_results =
        fetch_search_results_recursive(&args, args.staring_index, Vec::new()).await?;

    let results_count = search_results.len();
    let results_filename = temp::store_search_results(search_results)?;

    println!(
        "{}",
        serde_json::to_string(&SuccessResponse {
            error: false,
            next_start: args.staring_index + results_count,
            results_count,
            results_filename
        })
        .expect("serialization of json failed")
    );

    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Err(e) = tool(args).await {
        println!(
            "{}",
            serde_json::to_string(&ErrorResponse {
                error: true,
                message: e.to_string()
            })
            .expect("serialization of json failed")
        );
    }
}
