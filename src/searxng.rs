use crate::config::AppConfig;
use crate::error::AppError;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SearxngResponse {
    pub results: Vec<SearxngResult>,
    #[serde(default)]
    pub number_of_results: Option<u64>,
    #[serde(default)]
    pub suggestions: Vec<String>,
    #[serde(default)]
    pub answers: Vec<serde_json::Value>,
    #[serde(default)]
    pub corrections: Vec<String>,
    #[serde(default)]
    pub infoboxes: Vec<serde_json::Value>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SearxngResult {
    pub title: String,
    pub url: String,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub engine: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub published_date: Option<serde_json::Value>,
    #[serde(default)]
    pub thumbnail: Option<String>,
    #[serde(default)]
    pub img_src: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SearchParams {
    pub query: String,
    pub category: String,
    pub language: Option<String>,
    pub pageno: Option<u32>,
    pub time_range: Option<String>,
    pub safesearch: Option<u8>,
}

pub struct SearxngClient {
    http: reqwest::Client,
    base_url: String,
    default_lang: String,
    default_format: String,
}

impl SearxngClient {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            http: reqwest::Client::builder()
                .build()
                .expect("failed to build HTTP client"),
            base_url: config.searxng_url.trim_end_matches('/').to_string(),
            default_lang: config.default_lang.clone(),
            default_format: config.default_format.clone(),
        }
    }

    pub async fn search(&self, params: SearchParams) -> Result<SearxngResponse, AppError> {
        let mut query_params = vec![
            ("q", params.query),
            ("format", self.default_format.clone()),
            ("categories", params.category),
        ];

        let lang = params.language.unwrap_or_else(|| self.default_lang.clone());
        query_params.push(("language", lang));

        if let Some(pageno) = params.pageno {
            query_params.push(("pageno", pageno.to_string()));
        }

        if let Some(ref time_range) = params.time_range {
            query_params.push(("time_range", time_range.clone()));
        }

        if let Some(safesearch) = params.safesearch {
            query_params.push(("safesearch", safesearch.to_string()));
        }

        let url = format!("{}/search", self.base_url);
        let resp = self.http.get(&url).query(&query_params).send().await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::SearxngApi(format!("HTTP {status}: {body}")));
        }

        let response: SearxngResponse = resp.json().await?;
        Ok(response)
    }
}
