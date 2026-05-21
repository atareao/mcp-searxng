use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub searxng_url: String,
    pub transport: TransportMode,
    pub host: String,
    pub port: u16,
    pub default_lang: String,
    pub default_format: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransportMode {
    Stdio,
    Http,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let searxng_url =
            std::env::var("SEARXNG_URL").map_err(|_| AppError::MissingEnvVar("SEARXNG_URL"))?;

        let transport = match std::env::var("MCP_TRANSPORT").as_deref().unwrap_or("stdio") {
            "stdio" => TransportMode::Stdio,
            "http" => TransportMode::Http,
            other => {
                return Err(AppError::InvalidEnvVar(
                    "MCP_TRANSPORT",
                    format!("must be 'stdio' or 'http', got '{other}'"),
                ));
            }
        };

        let host = std::env::var("MCP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = std::env::var("MCP_PORT")
            .as_deref()
            .map(|s| s.parse::<u16>())
            .unwrap_or(Ok(3005))
            .map_err(|e| AppError::InvalidEnvVar("MCP_PORT", e.to_string()))?;

        let default_lang = std::env::var("SEARXNG_LANG").unwrap_or_else(|_| "es".to_string());

        let default_format = std::env::var("SEARXNG_FORMAT").unwrap_or_else(|_| "json".to_string());

        Ok(Self {
            searxng_url,
            transport,
            host,
            port,
            default_lang,
            default_format,
        })
    }
}
