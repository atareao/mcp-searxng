use std::sync::Arc;

use async_trait::async_trait;
use rust_mcp_sdk::McpServer;
use rust_mcp_sdk::mcp_server::ServerHandler;
use rust_mcp_sdk::schema::{
    CallToolRequestParams, CallToolResult, CompleteRequestParams, CompleteResult,
    ListResourceTemplatesResult, ListResourcesResult, ListToolsResult, PaginatedRequestParams,
    ReadResourceRequestParams, ReadResourceResult, RpcError, schema_utils::CallToolError,
};

use crate::config::AppConfig;
use crate::searxng::SearxngClient;

pub struct SearxngHandler {
    pub config: AppConfig,
    pub client: SearxngClient,
}

impl SearxngHandler {
    pub fn new(config: AppConfig) -> Self {
        let client = SearxngClient::new(&config);
        Self { config, client }
    }
}

#[async_trait]
impl ServerHandler for SearxngHandler {
    async fn handle_list_tools_request(
        &self,
        _params: Option<PaginatedRequestParams>,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            meta: None,
            next_cursor: None,
            tools: crate::tools::SearxngTools::tools(),
        })
    }

    async fn handle_call_tool_request(
        &self,
        params: CallToolRequestParams,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        let tool = crate::tools::SearxngTools::try_from(params).map_err(CallToolError::new)?;

        let config = &self.config;
        let client = &self.client;

        match tool {
            crate::tools::SearxngTools::Search(t) => t.call_tool(config, client).await,
            crate::tools::SearxngTools::SearchImages(t) => t.call_tool(config, client).await,
            crate::tools::SearxngTools::SearchVideos(t) => t.call_tool(config, client).await,
            crate::tools::SearxngTools::SearchNews(t) => t.call_tool(config, client).await,
            crate::tools::SearxngTools::SearchMusic(t) => t.call_tool(config, client).await,
            crate::tools::SearxngTools::SearchIt(t) => t.call_tool(config, client).await,
            crate::tools::SearxngTools::SearchScience(t) => t.call_tool(config, client).await,
            crate::tools::SearxngTools::SearchFiles(t) => t.call_tool(config, client).await,
            crate::tools::SearxngTools::SearchSocial(t) => t.call_tool(config, client).await,
            crate::tools::SearxngTools::SearchMap(t) => t.call_tool(config, client).await,
        }
    }

    async fn handle_list_resources_request(
        &self,
        _params: Option<PaginatedRequestParams>,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<ListResourcesResult, RpcError> {
        Ok(ListResourcesResult {
            meta: None,
            next_cursor: None,
            resources: vec![],
        })
    }

    async fn handle_list_resource_templates_request(
        &self,
        _params: Option<PaginatedRequestParams>,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<ListResourceTemplatesResult, RpcError> {
        Ok(ListResourceTemplatesResult {
            meta: None,
            next_cursor: None,
            resource_templates: vec![],
        })
    }

    async fn handle_read_resource_request(
        &self,
        _params: ReadResourceRequestParams,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<ReadResourceResult, RpcError> {
        Err(RpcError::method_not_found())
    }

    async fn handle_complete_request(
        &self,
        _params: CompleteRequestParams,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<CompleteResult, RpcError> {
        Err(RpcError::method_not_found())
    }
}
