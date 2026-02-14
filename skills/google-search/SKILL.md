---
name: google-search
description: Search Google and retrieve search results with pagination support
---

# Google Search Skill

## When to use this skill

Use this skill when the user needs to:
- Search Google for web results
- Get structured search result data including titles, descriptions, and links
- Paginate through search results beyond the first page

## Tool location

The binary is located at:
```
~/.local/bin/icrawl-google-search
```

## How to execute

Run the binary with the `--search` argument (required):

```bash
~/.local/bin/icrawl-google-search \
  --search "your search query here"
```

### Optional arguments

- `--staring-index <number>`: Starting index for pagination (default: 0). Use the `next_start` field from a previous search to continue.
- `--results-limit <number>`: Maximum number of results to fetch (default: 5)
- `--host-lang <string>`: Host language setting (default: "en-US")
- `--lang-restriction <string>`: Language restriction filter (default: "lang_en")

### Example with pagination

```bash
# First search
~/.local/bin/icrawl-google-search \
  --search "rust programming" \
  --results-limit 10

# Continue from next_start returned in previous response
~/.local/bin/icrawl-google-search \
  --search "rust programming" \
  --staring-index 11 \
  --results-limit 10
```

## Response format

The tool outputs JSON to stdout.

### Success response

```json
{
  "error": false,
  "next_start": 11,
  "results_count": 10,
  "results_filename": "/tmp/icrawl-google-search-cache/1234567890"
}
```

- `next_start`: Use this as `--staring-index` to fetch the next page
- `results_count`: Number of results returned
- `results_filename`: Path to YAML file containing the search results

### Error response

```json
{
  "error": true,
  "message": "Error description here"
}
```

## Reading search results

The `results_filename` points to a YAML file with this structure:

```yaml
search_results:
  - link: "https://example.com"
    image: "https://example.com/image.jpg"
    base_url: "example.com"
    title: "Example Title"
    description: "Example description text"
    lang: "en"
  - link: "https://another.com"
    title: "Another Title"
    # ... more fields (all optional except link/title/description)
```

All fields in each result are optional:
- `link`: Direct URL to the result
- `image`: Image thumbnail URL (if available)
- `base_url`: Base domain
- `title`: Result page title
- `description`: Meta description or snippet
- `lang`: Detected language code

To read the results, use the Read tool on the `results_filename` path returned.

## Important notes

- The tool caches ARC IDs and user agents for 1 hour to avoid detection
- Random delays (1-3 seconds) are added between paginated requests
- Results are stored as timestamped YAML files in the system temp directory
- If Google shows a captcha, the tool will return an error
