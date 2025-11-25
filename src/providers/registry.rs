use super::{
    error::ProviderError, AnthropicCompatibleProvider, AnthropicProvider, OpenAIProvider,
    ProviderConfig,
};
use crate::auth::TokenStore;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

/// Provider registry that manages all configured providers
pub struct ProviderRegistry {
    /// Map of provider name -> provider instance
    providers: HashMap<String, Arc<Box<dyn AnthropicProvider>>>,
    /// Map of model name -> provider name for fast lookup
    model_to_provider: HashMap<String, String>,
}

impl ProviderRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            model_to_provider: HashMap::new(),
        }
    }

    /// Load providers from configuration
    pub fn from_configs(
        configs: &[ProviderConfig],
        token_store: Option<TokenStore>,
    ) -> Result<Self, ProviderError> {
        let mut registry = Self::new();

        for config in configs {
            // Skip disabled providers
            if !config.is_enabled() {
                continue;
            }

            // Get API key - required for API key auth, skipped for OAuth
            let api_key = match &config.auth_type {
                super::AuthType::ApiKey => resolve_api_key(config)?,
                super::AuthType::OAuth => {
                    // OAuth providers will handle authentication differently
                    // For now, use a placeholder - will be replaced with token
                    config
                        .oauth_provider
                        .clone()
                        .unwrap_or_else(|| config.name.clone())
                }
            };

            // Create provider instance based on type
            let provider: Box<dyn AnthropicProvider> = match config.provider_type.as_str() {
                // OpenAI
                "openai" => Box::new(OpenAIProvider::new(
                    config.name.clone(),
                    api_key,
                    config
                        .base_url
                        .clone()
                        .unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
                    config.models.clone(),
                    config.oauth_provider.clone(),
                    token_store.clone(),
                )),

                // Anthropic-compatible providers
                "anthropic" => Box::new(AnthropicCompatibleProvider::new(
                    config.name.clone(),
                    api_key,
                    config
                        .base_url
                        .clone()
                        .unwrap_or_else(|| "https://api.anthropic.com".to_string()),
                    config.models.clone(),
                    config.oauth_provider.clone(),
                    token_store.clone(),
                )),
                "z.ai" => Box::new(AnthropicCompatibleProvider::zai(
                    api_key,
                    config.models.clone(),
                    token_store.clone(),
                )),
                "minimax" => Box::new(AnthropicCompatibleProvider::minimax(
                    api_key,
                    config.models.clone(),
                    token_store.clone(),
                )),
                "zenmux" => Box::new(AnthropicCompatibleProvider::zenmux(
                    api_key,
                    config.models.clone(),
                    token_store.clone(),
                )),
                "kimi-coding" => Box::new(AnthropicCompatibleProvider::kimi_coding(
                    api_key,
                    config.models.clone(),
                    token_store.clone(),
                )),

                // OpenAI-compatible providers
                "openrouter" => Box::new(OpenAIProvider::openrouter(
                    config.name.clone(),
                    api_key,
                    config.models.clone(),
                )),
                "deepinfra" => Box::new(OpenAIProvider::deepinfra(
                    config.name.clone(),
                    api_key,
                    config.models.clone(),
                )),
                "novita" => Box::new(OpenAIProvider::novita(
                    config.name.clone(),
                    api_key,
                    config.models.clone(),
                )),
                "baseten" => Box::new(OpenAIProvider::baseten(
                    config.name.clone(),
                    api_key,
                    config.models.clone(),
                )),
                "together" => Box::new(OpenAIProvider::together(
                    config.name.clone(),
                    api_key,
                    config.models.clone(),
                )),
                "github-copilot" | "copilot" => Box::new(OpenAIProvider::github_copilot(
                    config.name.clone(),
                    api_key,
                    config.models.clone(),
                )),
                "fireworks" => Box::new(OpenAIProvider::fireworks(
                    config.name.clone(),
                    api_key,
                    config.models.clone(),
                )),
                "groq" => Box::new(OpenAIProvider::groq(
                    config.name.clone(),
                    api_key,
                    config.models.clone(),
                )),
                "nebius" => Box::new(OpenAIProvider::nebius(
                    config.name.clone(),
                    api_key,
                    config.models.clone(),
                )),
                "cerebras" => Box::new(OpenAIProvider::cerebras(
                    config.name.clone(),
                    api_key,
                    config.models.clone(),
                )),
                "moonshot" => Box::new(OpenAIProvider::moonshot(
                    config.name.clone(),
                    api_key,
                    config.models.clone(),
                )),
                "qwen" => Box::new(OpenAIProvider::qwen(
                    config.name.clone(),
                    api_key,
                    config.models.clone(),
                )),
                "gemini" => Box::new(OpenAIProvider::gemini(
                    config.name.clone(),
                    api_key,
                    config.models.clone(),
                )),
                "longcat" => Box::new(OpenAIProvider::longcat(
                    config.name.clone(),
                    api_key,
                    config
                        .base_url
                        .clone()
                        .unwrap_or_else(|| "https://api.longcat.ai/v1".to_string()),
                    config.models.clone(),
                )),
                "ollama" => Box::new(OpenAIProvider::ollama(
                    config.name.clone(),
                    api_key,
                    config
                        .base_url
                        .clone()
                        .unwrap_or_else(|| "http://localhost:11434/v1".to_string()),
                    config.models.clone(),
                )),
                "lmstudio" => Box::new(OpenAIProvider::lmstudio(
                    config.name.clone(),
                    api_key,
                    config
                        .base_url
                        .clone()
                        .unwrap_or_else(|| "http://localhost:1234/v1".to_string()),
                    config.models.clone(),
                )),

                other => {
                    return Err(ProviderError::ConfigError(format!(
                        "Unknown provider type: {}",
                        other
                    )));
                }
            };

            // NOTE: models field in provider config is deprecated
            // Model mappings are now defined in [[models]] section
            // We only register the provider by name

            // Add provider to registry
            registry
                .providers
                .insert(config.name.clone(), Arc::new(provider));
        }

        Ok(registry)
    }

    /// Get a provider by name
    pub fn get_provider(&self, name: &str) -> Option<Arc<Box<dyn AnthropicProvider>>> {
        self.providers.get(name).cloned()
    }

    /// Get a provider for a specific model
    pub fn get_provider_for_model(
        &self,
        model: &str,
    ) -> Result<Arc<Box<dyn AnthropicProvider>>, ProviderError> {
        // First, check if we have a direct model → provider mapping
        if let Some(provider_name) = self.model_to_provider.get(model) {
            if let Some(provider) = self.providers.get(provider_name) {
                return Ok(provider.clone());
            }
        }

        // If no direct mapping, search through all providers
        for provider in self.providers.values() {
            if provider.supports_model(model) {
                return Ok(provider.clone());
            }
        }

        Err(ProviderError::ModelNotSupported(model.to_string()))
    }

    /// List all available models
    pub fn list_models(&self) -> Vec<String> {
        self.model_to_provider.keys().cloned().collect()
    }

    /// List all providers
    pub fn list_providers(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }
}

/// Resolve API key from direct value or CLI auth JSON path
fn resolve_api_key(config: &ProviderConfig) -> Result<String, ProviderError> {
    if let Some(ref api_key) = config.api_key {
        if !api_key.trim().is_empty() {
            return Ok(api_key.clone());
        }
    }

    if let Some(ref path) = config.api_key_path {
        return load_api_key_from_file(path).map_err(|e| {
            ProviderError::ConfigError(format!("Failed to read api_key from {}: {}", path, e))
        });
    }

    Err(ProviderError::ConfigError(format!(
        "Provider '{}' requires api_key or api_key_path for ApiKey auth",
        config.name
    )))
}

fn load_api_key_from_file(path_str: &str) -> Result<String, ProviderError> {
    let path = expand_home(path_str)
        .ok_or_else(|| ProviderError::ConfigError(format!("Invalid api_key_path: {}", path_str)))?;

    let content = fs::read_to_string(&path).map_err(|e| {
        ProviderError::ConfigError(format!("Failed to read {}: {}", path.display(), e))
    })?;

    let trimmed = content.trim();
    if trimmed.starts_with('{') {
        let value: Value = serde_json::from_str(trimmed).map_err(|e| {
            ProviderError::ConfigError(format!(
                "Failed to parse JSON from {}: {}",
                path.display(),
                e
            ))
        })?;

        extract_api_key_from_json(&value).ok_or_else(|| {
            ProviderError::ConfigError(format!(
                "No api_key/access_token/token field found in {}",
                path.display()
            ))
        })
    } else {
        Ok(trimmed.to_string())
    }
}

fn expand_home(path: &str) -> Option<PathBuf> {
    if let Some(stripped) = path.strip_prefix("~/") {
        dirs::home_dir().map(|home| home.join(stripped))
    } else {
        Some(PathBuf::from(path))
    }
}

fn extract_api_key_from_json(value: &Value) -> Option<String> {
    const TOKEN_KEYS: [&str; 6] = [
        "api_key",
        "token",
        "access_token",
        "key",
        "oauth_token",
        "bearer_token",
    ];

    fn search(value: &Value, token_keys: &[&str]) -> Option<String> {
        match value {
            Value::Object(map) => {
                for key in token_keys {
                    if let Some(v) = map.get(*key).and_then(|v| v.as_str()) {
                        let trimmed = v.trim();
                        if !trimmed.is_empty() {
                            return Some(trimmed.to_string());
                        }
                    }
                }

                for nested in map.values() {
                    if let Some(found) = search(nested, token_keys) {
                        return Some(found);
                    }
                }
                None
            }
            Value::Array(arr) => arr.iter().find_map(|v| search(v, token_keys)),
            _ => None,
        }
    }

    search(value, &TOKEN_KEYS)
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_registry() {
        let registry = ProviderRegistry::new();
        assert!(registry.list_models().is_empty());
        assert!(registry.list_providers().is_empty());
    }

    #[test]
    fn test_get_provider_for_model_not_found() {
        let registry = ProviderRegistry::new();
        let result = registry.get_provider_for_model("gpt-4");
        assert!(result.is_err());
    }
}
