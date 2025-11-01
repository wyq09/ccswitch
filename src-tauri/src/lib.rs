use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

// 新的Provider结构
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Provider {
    id: String,
    name: String,
    icon: Option<String>,
    #[serde(rename = "baseUrl")]
    base_url: String,
    #[serde(rename = "apiKey")]
    api_key: String,
    tags: Vec<String>,
    models: Models,
    #[serde(rename = "anthropicModel")]
    #[serde(default)]
    anthropic_model: Option<String>,
    #[serde(rename = "anthropicSmallFastModel")]
    #[serde(default)]
    anthropic_small_fast_model: Option<String>,
    #[serde(rename = "customEnv")]
    #[serde(default)]
    custom_env: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "isActive")]
    is_active: bool,
    #[serde(rename = "createdAt")]
    created_at: i64,
    #[serde(rename = "updatedAt")]
    updated_at: i64,
}

// 旧的Provider结构（用于迁移）
#[derive(Debug, Clone, Deserialize)]
struct ProviderV1 {
    id: String,
    name: String,
    icon: Option<String>,
    #[serde(rename = "baseUrl")]
    base_url: String,
    #[serde(rename = "apiKey")]
    api_key: String,
    tags: Vec<String>,
    models: ModelsV1,
    #[serde(rename = "isActive")]
    is_active: bool,
    #[serde(rename = "createdAt")]
    created_at: i64,
    #[serde(rename = "updatedAt")]
    updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Models {
    default: String,
    #[serde(rename = "smallFast")]
    small_fast: String,
    opus: String,
    sonnet: String,
    haiku: String,
}

// 旧的Models结构（只有三个字段）
#[derive(Debug, Clone, Deserialize)]
struct ModelsV1 {
    opus: String,
    sonnet: String,
    haiku: String,
}

impl ModelsV1 {
    fn migrate(self) -> Models {
        Models {
            default: self.sonnet.clone(),
            small_fast: self.haiku.clone(),
            opus: self.opus,
            sonnet: self.sonnet,
            haiku: self.haiku,
        }
    }
}

impl ProviderV1 {
    fn migrate(self) -> Provider {
        Provider {
            id: self.id,
            name: self.name,
            icon: self.icon,
            base_url: self.base_url,
            api_key: self.api_key,
            tags: self.tags,
            models: self.models.migrate(),
            anthropic_model: None,
            anthropic_small_fast_model: None,
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
            custom_env: serde_json::Map::new(), // Default to empty map
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Template {
    id: String,
    name: String,
    icon: String,
    #[serde(rename = "baseUrl")]
    base_url: String,
    #[serde(rename = "defaultModels")]
    default_models: TemplateModels,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TemplateModels {
    opus: String,
    sonnet: String,
    haiku: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClaudeSettings {
    env: serde_json::Value,
    #[serde(flatten)]
    other: serde_json::Map<String, serde_json::Value>,
}

fn get_app_data_dir(app: tauri::AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))
}

fn get_providers_file_path(app: tauri::AppHandle) -> Result<PathBuf, String> {
    let mut path = get_app_data_dir(app)?;
    fs::create_dir_all(&path).map_err(|e| format!("Failed to create app data dir: {}", e))?;
    path.push("providers.json");
    Ok(path)
}

fn get_templates_file_path(app: tauri::AppHandle) -> Result<PathBuf, String> {
    let mut path = get_app_data_dir(app)?;
    fs::create_dir_all(&path).map_err(|e| format!("Failed to create app data dir: {}", e))?;
    path.push("templates.json");
    Ok(path)
}

fn get_claude_settings_path() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("Failed to get home directory")?;
    let mut path = home;
    path.push(".claude");
    path.push("settings.json");
    Ok(path)
}

#[tauri::command]
fn load_providers(app: tauri::AppHandle) -> Result<Vec<Provider>, String> {
    let path = get_providers_file_path(app)?;
    
    if !path.exists() {
        return Ok(Vec::new());
    }
    
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read providers file: {}", e))?;
    
    // 首先尝试用新格式解析
    match serde_json::from_str::<Vec<Provider>>(&content) {
        Ok(providers) => Ok(providers),
        Err(_) => {
            // 如果新格式失败，尝试用旧格式解析并迁移
            match serde_json::from_str::<Vec<ProviderV1>>(&content) {
                Ok(old_providers) => {
                    let providers: Vec<Provider> = old_providers
                        .into_iter()
                        .map(|p| p.migrate())
                        .collect();
                    
                    // 自动保存迁移后的数据
                    let new_content = serde_json::to_string_pretty(&providers)
                        .map_err(|e| format!("Failed to serialize providers: {}", e))?;
                    
                    fs::write(&path, new_content)
                        .map_err(|e| format!("Failed to write migrated providers: {}", e))?;
                    
                    Ok(providers)
                }
                Err(e) => Err(format!("Failed to parse providers file: {}", e))
            }
        }
    }
}

#[tauri::command]
fn save_provider(app: tauri::AppHandle, provider: Provider) -> Result<(), String> {
    let path = get_providers_file_path(app.clone())?;
    let mut providers = load_providers(app)?;
    
    // Find and update or add new
    if let Some(pos) = providers.iter().position(|p| p.id == provider.id) {
        providers[pos] = provider;
    } else {
        providers.push(provider);
    }
    
    let content = serde_json::to_string_pretty(&providers)
        .map_err(|e| format!("Failed to serialize providers: {}", e))?;
    
    fs::write(&path, content)
        .map_err(|e| format!("Failed to write providers file: {}", e))?;
    
    Ok(())
}

#[tauri::command]
fn delete_provider(app: tauri::AppHandle, id: String) -> Result<(), String> {
    let path = get_providers_file_path(app.clone())?;
    let mut providers = load_providers(app)?;
    
    providers.retain(|p| p.id != id);
    
    let content = serde_json::to_string_pretty(&providers)
        .map_err(|e| format!("Failed to serialize providers: {}", e))?;
    
    fs::write(&path, content)
        .map_err(|e| format!("Failed to write providers file: {}", e))?;
    
    Ok(())
}

#[tauri::command]
fn switch_provider(app: tauri::AppHandle, id: String) -> Result<(), String> {
    let path = get_providers_file_path(app.clone())?;
    let mut providers = load_providers(app)?;
    
    // Find the provider to activate
    let provider = providers
        .iter()
        .find(|p| p.id == id)
        .ok_or("Provider not found")?
        .clone();
    
    // Update active status
    for p in &mut providers {
        p.is_active = p.id == id;
    }
    
    // Save updated providers
    let content = serde_json::to_string_pretty(&providers)
        .map_err(|e| format!("Failed to serialize providers: {}", e))?;
    
    fs::write(&path, content)
        .map_err(|e| format!("Failed to write providers file: {}", e))?;
    
    // Update Claude settings
    update_claude_settings(&provider)?;
    
    Ok(())
}

fn update_claude_settings(provider: &Provider) -> Result<(), String> {
    let settings_path = get_claude_settings_path()?;
    
    // Create .claude directory if it doesn't exist
    if let Some(parent) = settings_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create .claude directory: {}", e))?;
    }
    
    // Read existing settings or create new
    let mut settings: ClaudeSettings = if settings_path.exists() {
        let content = fs::read_to_string(&settings_path)
            .map_err(|e| format!("Failed to read Claude settings: {}", e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse Claude settings: {}", e))?
    } else {
        ClaudeSettings {
            env: serde_json::json!({}),
            other: serde_json::Map::new(),
        }
    };
    
    // Update env variables
    let env_obj = settings.env.as_object_mut()
        .ok_or("env is not an object")?;
    
    env_obj.insert("ANTHROPIC_BASE_URL".to_string(), serde_json::json!(provider.base_url));
    env_obj.insert("ANTHROPIC_AUTH_TOKEN".to_string(), serde_json::json!(provider.api_key));
    env_obj.insert("API_TIMEOUT_MS".to_string(), serde_json::json!("3000000"));
    env_obj.insert("CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC".to_string(), serde_json::json!(1));
    
    // Use anthropicModel and anthropicSmallFastModel if provided, otherwise use defaults
    let anthropic_model = provider.anthropic_model.as_deref().unwrap_or(&provider.models.default);
    let anthropic_small_fast = provider.anthropic_small_fast_model.as_deref().unwrap_or(&provider.models.small_fast);
    
    env_obj.insert("ANTHROPIC_MODEL".to_string(), serde_json::json!(anthropic_model));
    env_obj.insert("ANTHROPIC_SMALL_FAST_MODEL".to_string(), serde_json::json!(anthropic_small_fast));
    env_obj.insert("ANTHROPIC_DEFAULT_SONNET_MODEL".to_string(), serde_json::json!(&provider.models.sonnet));
    env_obj.insert("ANTHROPIC_DEFAULT_OPUS_MODEL".to_string(), serde_json::json!(&provider.models.opus));
    env_obj.insert("ANTHROPIC_DEFAULT_HAIKU_MODEL".to_string(), serde_json::json!(&provider.models.haiku));
    
    // Add custom environment variables
    for (key, value) in provider.custom_env.iter() {
        env_obj.insert(key.clone(), value.clone());
    }
    
    // Write back to file
    let content = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize Claude settings: {}", e))?;
    
    fs::write(&settings_path, content)
        .map_err(|e| format!("Failed to write Claude settings: {}", e))?;
    
    Ok(())
}

#[tauri::command]
fn load_templates(app: tauri::AppHandle) -> Result<Vec<Template>, String> {
    let path = get_templates_file_path(app)?;
    
    if !path.exists() {
        return Ok(Vec::new());
    }
    
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read templates file: {}", e))?;
    
    let templates: Vec<Template> = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse templates file: {}", e))?;
    
    Ok(templates)
}

#[tauri::command]
fn save_template(app: tauri::AppHandle, template: Template) -> Result<(), String> {
    let path = get_templates_file_path(app.clone())?;
    let mut templates = load_templates(app)?;
    
    // Find and update or add new
    if let Some(pos) = templates.iter().position(|t| t.id == template.id) {
        templates[pos] = template;
    } else {
        templates.push(template);
    }
    
    let content = serde_json::to_string_pretty(&templates)
        .map_err(|e| format!("Failed to serialize templates: {}", e))?;
    
    fs::write(&path, content)
        .map_err(|e| format!("Failed to write templates file: {}", e))?;
    
    Ok(())
}

#[tauri::command]
fn delete_template(app: tauri::AppHandle, id: String) -> Result<(), String> {
    let path = get_templates_file_path(app.clone())?;
    let mut templates = load_templates(app)?;
    
    templates.retain(|t| t.id != id);
    
    let content = serde_json::to_string_pretty(&templates)
        .map_err(|e| format!("Failed to serialize templates: {}", e))?;
    
    fs::write(&path, content)
        .map_err(|e| format!("Failed to write templates file: {}", e))?;
    
    Ok(())
}

#[tauri::command]
fn clear_all_providers(app: tauri::AppHandle) -> Result<(), String> {
    let path = get_providers_file_path(app)?;
    
    // Clear the providers file
    fs::write(&path, "[]")
        .map_err(|e| format!("Failed to clear providers file: {}", e))?;
    
    // Clear Claude settings
    let settings_path = get_claude_settings_path()?;
    
    if settings_path.exists() {
        // Read existing settings and clear the env object
        let mut settings: ClaudeSettings = if settings_path.exists() {
            let content = fs::read_to_string(&settings_path)
                .map_err(|e| format!("Failed to read Claude settings: {}", e))?;
            serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse Claude settings: {}", e))?
        } else {
            ClaudeSettings {
                env: serde_json::json!({}),
                other: serde_json::Map::new(),
            }
        };
        
        // Clear all env variables
        if let Some(env_obj) = settings.env.as_object_mut() {
            env_obj.clear();
        }
        
        // Write back to file
        let content = serde_json::to_string_pretty(&settings)
            .map_err(|e| format!("Failed to serialize Claude settings: {}", e))?;
        
        fs::write(&settings_path, content)
            .map_err(|e| format!("Failed to write Claude settings: {}", e))?;
    }
    
    Ok(())
}

#[tauri::command]
fn get_claude_settings_path_cmd() -> Result<String, String> {
    let path = get_claude_settings_path()?;
    Ok(path.to_string_lossy().to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            load_providers,
            save_provider,
            delete_provider,
            switch_provider,
            load_templates,
            save_template,
            delete_template,
            clear_all_providers,
            get_claude_settings_path_cmd,
        ])
        .setup(|_app| {
            // Use system default window decorations (native title bar & controls)
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
