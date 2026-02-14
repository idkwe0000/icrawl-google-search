# Type Definitions Reference

This document describes the Rust structs used by the icrawl-google-search tool.

## Args (CLI Arguments)

```rust
pub struct Args {
    pub search: String,           // Required: What you want to search
    pub staring_index: usize,      // Default: 0 - Starting index for pagination
    pub results_limit: usize,      // Default: 5 - Max results to fetch
    pub host_lang: String,         // Default: "en-US" - Host language
    pub lang_restriction: String,  // Default: "lang_en" - Language filter
}
```

## SearchRequestResponseEntry (Search Result)

Each search result item contains:

```rust
pub struct SearchRequestResponseEntry {
    pub link: Option<String>,        // Direct URL to the result
    pub image: Option<String>,       // Image thumbnail URL
    pub base_url: Option<String>,    // Base domain
    pub title: Option<String>,        // Page title
    pub description: Option<String>,  // Meta description/snippet
    pub lang: Option<String>,        // Detected language code
}
```

All fields are optional - some results may only have a few fields populated.

## SuccessResponse

```json
{
  "error": false,
  "next_start": 11,           // Use this for staring_index to paginate
  "results_count": 10,         // Number of results returned
  "results_filename": "/tmp/..." // Path to YAML results file
}
```

## ErrorResponse

```json
{
  "error": true,
  "message": "Error description"
}
```
