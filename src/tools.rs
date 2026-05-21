use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
use rust_mcp_sdk::schema::{CallToolResult, TextContent, schema_utils::CallToolError};

use crate::config::AppConfig;
use crate::searxng::{SearchParams, SearxngClient};

// ── search (general) ──────────────────────────────────────────────────────────

#[mcp_tool(
    name = "search",
    description = "General web search across all categories",
    read_only_hint = true
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct Search {
    /// The search query
    query: String,
    /// Language code (e.g. 'en', 'es'). Overrides the default.
    language: Option<String>,
    /// Page number (default: 1)
    pageno: Option<u32>,
    /// Time filter: 'day', 'month', or 'year'
    time_range: Option<String>,
    /// Safe search level: 0 (off), 1 (moderate), 2 (strict)
    safesearch: Option<u8>,
}

impl Search {
    pub async fn call_tool(
        &self,
        _config: &AppConfig,
        client: &SearxngClient,
    ) -> Result<CallToolResult, CallToolError> {
        let params = SearchParams {
            query: self.query.clone(),
            category: "general".to_string(),
            language: self.language.clone(),
            pageno: self.pageno,
            time_range: self.time_range.clone(),
            safesearch: self.safesearch,
        };
        let response = client
            .search(params)
            .await
            .map_err(|e| CallToolError::from_message(e.to_string()))?;
        Ok(CallToolResult::text_content(vec![TextContent::from(
            format_results(&response, "general"),
        )]))
    }
}

// ── search_images ─────────────────────────────────────────────────────────────

#[mcp_tool(
    name = "search_images",
    description = "Search for images",
    read_only_hint = true
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct SearchImages {
    /// The search query
    query: String,
    /// Language code (e.g. 'en', 'es'). Overrides the default.
    language: Option<String>,
    /// Page number (default: 1)
    pageno: Option<u32>,
    /// Time filter: 'day', 'month', or 'year'
    time_range: Option<String>,
    /// Safe search level: 0 (off), 1 (moderate), 2 (strict)
    safesearch: Option<u8>,
}

impl SearchImages {
    pub async fn call_tool(
        &self,
        _config: &AppConfig,
        client: &SearxngClient,
    ) -> Result<CallToolResult, CallToolError> {
        let params = SearchParams {
            query: self.query.clone(),
            category: "images".to_string(),
            language: self.language.clone(),
            pageno: self.pageno,
            time_range: self.time_range.clone(),
            safesearch: self.safesearch,
        };
        let response = client
            .search(params)
            .await
            .map_err(|e| CallToolError::from_message(e.to_string()))?;
        Ok(CallToolResult::text_content(vec![TextContent::from(
            format_results(&response, "images"),
        )]))
    }
}

// ── search_videos ─────────────────────────────────────────────────────────────

#[mcp_tool(
    name = "search_videos",
    description = "Search for videos",
    read_only_hint = true
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct SearchVideos {
    /// The search query
    query: String,
    /// Language code (e.g. 'en', 'es'). Overrides the default.
    language: Option<String>,
    /// Page number (default: 1)
    pageno: Option<u32>,
    /// Time filter: 'day', 'month', or 'year'
    time_range: Option<String>,
    /// Safe search level: 0 (off), 1 (moderate), 2 (strict)
    safesearch: Option<u8>,
}

impl SearchVideos {
    pub async fn call_tool(
        &self,
        _config: &AppConfig,
        client: &SearxngClient,
    ) -> Result<CallToolResult, CallToolError> {
        let params = SearchParams {
            query: self.query.clone(),
            category: "videos".to_string(),
            language: self.language.clone(),
            pageno: self.pageno,
            time_range: self.time_range.clone(),
            safesearch: self.safesearch,
        };
        let response = client
            .search(params)
            .await
            .map_err(|e| CallToolError::from_message(e.to_string()))?;
        Ok(CallToolResult::text_content(vec![TextContent::from(
            format_results(&response, "videos"),
        )]))
    }
}

// ── search_news ───────────────────────────────────────────────────────────────

#[mcp_tool(
    name = "search_news",
    description = "Search for news articles",
    read_only_hint = true
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct SearchNews {
    /// The search query
    query: String,
    /// Language code (e.g. 'en', 'es'). Overrides the default.
    language: Option<String>,
    /// Page number (default: 1)
    pageno: Option<u32>,
    /// Time filter: 'day', 'month', or 'year'
    time_range: Option<String>,
    /// Safe search level: 0 (off), 1 (moderate), 2 (strict)
    safesearch: Option<u8>,
}

impl SearchNews {
    pub async fn call_tool(
        &self,
        _config: &AppConfig,
        client: &SearxngClient,
    ) -> Result<CallToolResult, CallToolError> {
        let params = SearchParams {
            query: self.query.clone(),
            category: "news".to_string(),
            language: self.language.clone(),
            pageno: self.pageno,
            time_range: self.time_range.clone(),
            safesearch: self.safesearch,
        };
        let response = client
            .search(params)
            .await
            .map_err(|e| CallToolError::from_message(e.to_string()))?;
        Ok(CallToolResult::text_content(vec![TextContent::from(
            format_results(&response, "news"),
        )]))
    }
}

// ── search_music ──────────────────────────────────────────────────────────────

#[mcp_tool(
    name = "search_music",
    description = "Search for music",
    read_only_hint = true
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct SearchMusic {
    /// The search query
    query: String,
    /// Language code (e.g. 'en', 'es'). Overrides the default.
    language: Option<String>,
    /// Page number (default: 1)
    pageno: Option<u32>,
    /// Time filter: 'day', 'month', or 'year'
    time_range: Option<String>,
    /// Safe search level: 0 (off), 1 (moderate), 2 (strict)
    safesearch: Option<u8>,
}

impl SearchMusic {
    pub async fn call_tool(
        &self,
        _config: &AppConfig,
        client: &SearxngClient,
    ) -> Result<CallToolResult, CallToolError> {
        let params = SearchParams {
            query: self.query.clone(),
            category: "music".to_string(),
            language: self.language.clone(),
            pageno: self.pageno,
            time_range: self.time_range.clone(),
            safesearch: self.safesearch,
        };
        let response = client
            .search(params)
            .await
            .map_err(|e| CallToolError::from_message(e.to_string()))?;
        Ok(CallToolResult::text_content(vec![TextContent::from(
            format_results(&response, "music"),
        )]))
    }
}

// ── search_it ─────────────────────────────────────────────────────────────────

#[mcp_tool(
    name = "search_it",
    description = "Search for IT and technology topics",
    read_only_hint = true
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct SearchIt {
    /// The search query
    query: String,
    /// Language code (e.g. 'en', 'es'). Overrides the default.
    language: Option<String>,
    /// Page number (default: 1)
    pageno: Option<u32>,
    /// Time filter: 'day', 'month', or 'year'
    time_range: Option<String>,
    /// Safe search level: 0 (off), 1 (moderate), 2 (strict)
    safesearch: Option<u8>,
}

impl SearchIt {
    pub async fn call_tool(
        &self,
        _config: &AppConfig,
        client: &SearxngClient,
    ) -> Result<CallToolResult, CallToolError> {
        let params = SearchParams {
            query: self.query.clone(),
            category: "it".to_string(),
            language: self.language.clone(),
            pageno: self.pageno,
            time_range: self.time_range.clone(),
            safesearch: self.safesearch,
        };
        let response = client
            .search(params)
            .await
            .map_err(|e| CallToolError::from_message(e.to_string()))?;
        Ok(CallToolResult::text_content(vec![TextContent::from(
            format_results(&response, "it"),
        )]))
    }
}

// ── search_science ────────────────────────────────────────────────────────────

#[mcp_tool(
    name = "search_science",
    description = "Search for scientific and academic topics",
    read_only_hint = true
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct SearchScience {
    /// The search query
    query: String,
    /// Language code (e.g. 'en', 'es'). Overrides the default.
    language: Option<String>,
    /// Page number (default: 1)
    pageno: Option<u32>,
    /// Time filter: 'day', 'month', or 'year'
    time_range: Option<String>,
    /// Safe search level: 0 (off), 1 (moderate), 2 (strict)
    safesearch: Option<u8>,
}

impl SearchScience {
    pub async fn call_tool(
        &self,
        _config: &AppConfig,
        client: &SearxngClient,
    ) -> Result<CallToolResult, CallToolError> {
        let params = SearchParams {
            query: self.query.clone(),
            category: "science".to_string(),
            language: self.language.clone(),
            pageno: self.pageno,
            time_range: self.time_range.clone(),
            safesearch: self.safesearch,
        };
        let response = client
            .search(params)
            .await
            .map_err(|e| CallToolError::from_message(e.to_string()))?;
        Ok(CallToolResult::text_content(vec![TextContent::from(
            format_results(&response, "science"),
        )]))
    }
}

// ── search_files ──────────────────────────────────────────────────────────────

#[mcp_tool(
    name = "search_files",
    description = "Search for downloadable files",
    read_only_hint = true
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct SearchFiles {
    /// The search query
    query: String,
    /// Language code (e.g. 'en', 'es'). Overrides the default.
    language: Option<String>,
    /// Page number (default: 1)
    pageno: Option<u32>,
    /// Time filter: 'day', 'month', or 'year'
    time_range: Option<String>,
    /// Safe search level: 0 (off), 1 (moderate), 2 (strict)
    safesearch: Option<u8>,
}

impl SearchFiles {
    pub async fn call_tool(
        &self,
        _config: &AppConfig,
        client: &SearxngClient,
    ) -> Result<CallToolResult, CallToolError> {
        let params = SearchParams {
            query: self.query.clone(),
            category: "files".to_string(),
            language: self.language.clone(),
            pageno: self.pageno,
            time_range: self.time_range.clone(),
            safesearch: self.safesearch,
        };
        let response = client
            .search(params)
            .await
            .map_err(|e| CallToolError::from_message(e.to_string()))?;
        Ok(CallToolResult::text_content(vec![TextContent::from(
            format_results(&response, "files"),
        )]))
    }
}

// ── search_social ─────────────────────────────────────────────────────────────

#[mcp_tool(
    name = "search_social",
    description = "Search social media content",
    read_only_hint = true
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct SearchSocial {
    /// The search query
    query: String,
    /// Language code (e.g. 'en', 'es'). Overrides the default.
    language: Option<String>,
    /// Page number (default: 1)
    pageno: Option<u32>,
    /// Time filter: 'day', 'month', or 'year'
    time_range: Option<String>,
    /// Safe search level: 0 (off), 1 (moderate), 2 (strict)
    safesearch: Option<u8>,
}

impl SearchSocial {
    pub async fn call_tool(
        &self,
        _config: &AppConfig,
        client: &SearxngClient,
    ) -> Result<CallToolResult, CallToolError> {
        let params = SearchParams {
            query: self.query.clone(),
            category: "social media".to_string(),
            language: self.language.clone(),
            pageno: self.pageno,
            time_range: self.time_range.clone(),
            safesearch: self.safesearch,
        };
        let response = client
            .search(params)
            .await
            .map_err(|e| CallToolError::from_message(e.to_string()))?;
        Ok(CallToolResult::text_content(vec![TextContent::from(
            format_results(&response, "social media"),
        )]))
    }
}

// ── search_map ────────────────────────────────────────────────────────────────

#[mcp_tool(
    name = "search_map",
    description = "Search for locations and map data",
    read_only_hint = true
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct SearchMap {
    /// The search query
    query: String,
    /// Language code (e.g. 'en', 'es'). Overrides the default.
    language: Option<String>,
    /// Page number (default: 1)
    pageno: Option<u32>,
    /// Time filter: 'day', 'month', or 'year'
    time_range: Option<String>,
    /// Safe search level: 0 (off), 1 (moderate), 2 (strict)
    safesearch: Option<u8>,
}

impl SearchMap {
    pub async fn call_tool(
        &self,
        _config: &AppConfig,
        client: &SearxngClient,
    ) -> Result<CallToolResult, CallToolError> {
        let params = SearchParams {
            query: self.query.clone(),
            category: "map".to_string(),
            language: self.language.clone(),
            pageno: self.pageno,
            time_range: self.time_range.clone(),
            safesearch: self.safesearch,
        };
        let response = client
            .search(params)
            .await
            .map_err(|e| CallToolError::from_message(e.to_string()))?;
        Ok(CallToolResult::text_content(vec![TextContent::from(
            format_results(&response, "map"),
        )]))
    }
}

// ── toolbox ───────────────────────────────────────────────────────────────────

use rust_mcp_sdk::tool_box;

tool_box!(
    SearxngTools,
    [
        Search,
        SearchImages,
        SearchVideos,
        SearchNews,
        SearchMusic,
        SearchIt,
        SearchScience,
        SearchFiles,
        SearchSocial,
        SearchMap,
    ]
);

// ── formatting ────────────────────────────────────────────────────────────────

use crate::searxng::SearxngResponse;

fn format_results(response: &SearxngResponse, category: &str) -> String {
    if response.results.is_empty() {
        return "No results found.".to_string();
    }

    let mut text = format!("## Search Results ({})\n\n", category);

    if let Some(count) = response.number_of_results {
        text.push_str(&format!("**Total results:** {}\n\n", count));
    }

    for (i, result) in response.results.iter().enumerate() {
        text.push_str(&format!(
            "### {}. {}\n\n",
            i + 1,
            escape_markdown(&result.title)
        ));

        if let Some(ref content) = result.content {
            text.push_str(&format!("{}\n\n", escape_markdown(content)));
        }

        text.push_str(&format!("- **URL:** {}\n", result.url));

        if let Some(ref engine) = result.engine {
            text.push_str(&format!("- **Engine:** {}\n", engine));
        }

        if let Some(ref thumbnail) = result.thumbnail {
            text.push_str(&format!("- **Thumbnail:** {}\n", thumbnail));
        }

        if let Some(ref img_src) = result.img_src {
            text.push_str(&format!("- **Image:** {}\n", img_src));
        }

        text.push('\n');
    }

    if !response.suggestions.is_empty() {
        text.push_str("### Suggestions\n\n");
        for suggestion in &response.suggestions {
            text.push_str(&format!("- {}\n", escape_markdown(suggestion)));
        }
        text.push('\n');
    }

    text
}

fn escape_markdown(s: &str) -> String {
    s.replace('*', "\\*")
        .replace('_', "\\_")
        .replace('[', "\\[")
        .replace(']', "\\]")
}
