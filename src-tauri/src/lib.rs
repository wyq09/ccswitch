use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Provider {
    id: String,
    name: String,
    icon: Option<String>,
    #[serde(rename = "baseUrl")]
    base_url: String,
    #[serde(rename = "apiKey")]
    api_key: String,
    #[serde(default)]
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
    #[serde(default)]
    default: String,
    #[serde(rename = "smallFast")]
    #[serde(default)]
    small_fast: String,
    #[serde(default)]
    opus: String,
    #[serde(default)]
    sonnet: String,
    #[serde(default)]
    haiku: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ModelsV1 {
    opus: String,
    sonnet: String,
    haiku: String,
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

#[derive(Debug, Clone, Default)]
struct ClaudeSettings {
    env: serde_json::Map<String, serde_json::Value>,
    other: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProviderTransferFile {
    version: u32,
    providers: Vec<Provider>,
}

#[derive(Debug, Deserialize)]
struct ProviderTransferFileV1 {
    providers: Vec<ProviderV1>,
}

const PROVIDER_TRANSFER_VERSION: u32 = 2;

const MANAGED_CLAUDE_ENV_KEYS: [&str; 12] = [
    "ANTHROPIC_BASE_URL",
    "ANTHROPIC_AUTH_TOKEN",
    "ANTHROPIC_API_KEY",
    "API_TIMEOUT_MS",
    "AUTOPILOT",
    "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC",
    "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS",
    "ANTHROPIC_MODEL",
    "ANTHROPIC_SMALL_FAST_MODEL",
    "ANTHROPIC_DEFAULT_SONNET_MODEL",
    "ANTHROPIC_DEFAULT_OPUS_MODEL",
    "ANTHROPIC_DEFAULT_HAIKU_MODEL",
];

fn default_custom_env() -> serde_json::Map<String, serde_json::Value> {
    let mut env = serde_json::Map::new();
    env.insert("API_TIMEOUT_MS".to_string(), serde_json::json!("3000000"));
    env.insert("AUTOPILOT".to_string(), serde_json::json!("true"));
    env.insert(
        "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC".to_string(),
        serde_json::json!("1"),
    );
    env.insert(
        "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS".to_string(),
        serde_json::json!("1"),
    );
    env
}

fn merge_default_custom_env(custom_env: &mut serde_json::Map<String, serde_json::Value>) {
    for (key, value) in default_custom_env() {
        custom_env.entry(key).or_insert(value);
    }
}

impl Models {
    fn normalize(mut self) -> Self {
        if self.default.trim().is_empty() {
            self.default = self.sonnet.clone();
        }
        if self.small_fast.trim().is_empty() {
            self.small_fast = self.haiku.clone();
        }
        self
    }
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

impl Provider {
    fn normalize(mut self) -> Self {
        self.models = self.models.normalize();
        merge_default_custom_env(&mut self.custom_env);
        self
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
            custom_env: serde_json::Map::new(),
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
        .normalize()
    }
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

fn parse_providers_content(content: &str) -> Result<Vec<Provider>, String> {
    if let Ok(providers) = serde_json::from_str::<Vec<Provider>>(content) {
        return Ok(providers.into_iter().map(Provider::normalize).collect());
    }

    if let Ok(payload) = serde_json::from_str::<ProviderTransferFile>(content) {
        return Ok(payload
            .providers
            .into_iter()
            .map(Provider::normalize)
            .collect());
    }

    if let Ok(old_providers) = serde_json::from_str::<Vec<ProviderV1>>(content) {
        return Ok(old_providers.into_iter().map(ProviderV1::migrate).collect());
    }

    if let Ok(payload) = serde_json::from_str::<ProviderTransferFileV1>(content) {
        return Ok(payload
            .providers
            .into_iter()
            .map(ProviderV1::migrate)
            .collect());
    }

    let err = serde_json::from_str::<serde_json::Value>(content)
        .err()
        .map(|e| e.to_string())
        .unwrap_or_else(|| "Unknown provider file format".to_string());
    Err(format!("Failed to parse providers file: {}", err))
}

fn ensure_single_active_provider(providers: &mut [Provider]) {
    let active_ids = providers
        .iter()
        .filter(|provider| provider.is_active)
        .map(|provider| provider.id.clone())
        .collect::<Vec<_>>();

    if active_ids.len() <= 1 {
        return;
    }

    if let Some(active_id) = active_ids.last() {
        for provider in providers.iter_mut() {
            provider.is_active = provider.id == *active_id;
        }
    }
}

fn load_providers_from_path(path: &Path) -> Result<Vec<Provider>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read providers file: {}", e))?;

    if content.trim().is_empty() {
        return Ok(Vec::new());
    }

    let mut providers = parse_providers_content(&content)?;
    ensure_single_active_provider(&mut providers);

    let normalized_content = serde_json::to_string_pretty(&providers)
        .map_err(|e| format!("Failed to serialize providers: {}", e))?;
    if content.trim() != normalized_content {
        fs::write(path, normalized_content)
            .map_err(|e| format!("Failed to write migrated providers: {}", e))?;
    }

    Ok(providers)
}

fn write_providers_to_path(path: &Path, providers: &[Provider]) -> Result<(), String> {
    let content = serde_json::to_string_pretty(providers)
        .map_err(|e| format!("Failed to serialize providers: {}", e))?;
    fs::write(path, content).map_err(|e| format!("Failed to write providers file: {}", e))
}

fn managed_env_keys_for_provider(provider: Option<&Provider>) -> BTreeSet<String> {
    let mut keys = MANAGED_CLAUDE_ENV_KEYS
        .iter()
        .map(|key| key.to_string())
        .collect::<BTreeSet<_>>();

    if let Some(provider) = provider {
        for key in provider.custom_env.keys() {
            keys.insert(key.clone());
        }
    }

    keys
}

fn load_claude_settings(settings_path: &Path) -> Result<ClaudeSettings, String> {
    if !settings_path.exists() {
        return Ok(ClaudeSettings::default());
    }

    let content = fs::read_to_string(settings_path)
        .map_err(|e| format!("Failed to read Claude settings: {}", e))?;

    if content.trim().is_empty() {
        return Ok(ClaudeSettings::default());
    }

    let value: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse Claude settings: {}", e))?;
    let mut root = value
        .as_object()
        .cloned()
        .ok_or("Claude settings root must be a JSON object")?;

    let env = root
        .remove("env")
        .and_then(|env| env.as_object().cloned())
        .unwrap_or_default();

    Ok(ClaudeSettings { env, other: root })
}

fn save_claude_settings(settings_path: &Path, settings: ClaudeSettings) -> Result<(), String> {
    let mut root = settings.other;
    root.insert("env".to_string(), serde_json::Value::Object(settings.env));

    let content = serde_json::to_string_pretty(&serde_json::Value::Object(root))
        .map_err(|e| format!("Failed to serialize Claude settings: {}", e))?;

    fs::write(settings_path, content)
        .map_err(|e| format!("Failed to write Claude settings: {}", e))?;
    Ok(())
}

fn clear_managed_claude_settings(providers: &[Provider]) -> Result<(), String> {
    let settings_path = get_claude_settings_path()?;
    if !settings_path.exists() {
        return Ok(());
    }

    let mut settings = load_claude_settings(&settings_path)?;
    let mut keys = managed_env_keys_for_provider(None);
    for provider in providers {
        for key in managed_env_keys_for_provider(Some(provider)) {
            keys.insert(key);
        }
    }

    for key in keys {
        settings.env.remove(&key);
    }

    save_claude_settings(&settings_path, settings)
}

fn update_claude_settings(
    previous_provider: Option<&Provider>,
    provider: &Provider,
) -> Result<(), String> {
    let settings_path = get_claude_settings_path()?;
    if let Some(parent) = settings_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create .claude directory: {}", e))?;
    }

    let mut settings = load_claude_settings(&settings_path)?;
    for key in managed_env_keys_for_provider(previous_provider) {
        settings.env.remove(&key);
    }
    for key in managed_env_keys_for_provider(Some(provider)) {
        settings.env.remove(&key);
    }

    settings.env.insert(
        "ANTHROPIC_BASE_URL".to_string(),
        serde_json::json!(provider.base_url),
    );
    settings.env.insert(
        "ANTHROPIC_AUTH_TOKEN".to_string(),
        serde_json::json!(provider.api_key),
    );

    let anthropic_model = provider
        .anthropic_model
        .as_deref()
        .filter(|model| !model.trim().is_empty())
        .unwrap_or(&provider.models.default);
    let anthropic_small_fast_model = provider
        .anthropic_small_fast_model
        .as_deref()
        .filter(|model| !model.trim().is_empty())
        .unwrap_or(&provider.models.small_fast);

    settings.env.insert(
        "ANTHROPIC_MODEL".to_string(),
        serde_json::json!(anthropic_model),
    );
    settings.env.insert(
        "ANTHROPIC_SMALL_FAST_MODEL".to_string(),
        serde_json::json!(anthropic_small_fast_model),
    );
    settings.env.insert(
        "ANTHROPIC_DEFAULT_SONNET_MODEL".to_string(),
        serde_json::json!(provider.models.sonnet),
    );
    settings.env.insert(
        "ANTHROPIC_DEFAULT_OPUS_MODEL".to_string(),
        serde_json::json!(provider.models.opus),
    );
    settings.env.insert(
        "ANTHROPIC_DEFAULT_HAIKU_MODEL".to_string(),
        serde_json::json!(provider.models.haiku),
    );

    for (key, value) in provider.custom_env.iter() {
        settings.env.insert(key.clone(), value.clone());
    }

    save_claude_settings(&settings_path, settings)
}

#[tauri::command]
fn load_providers(app: tauri::AppHandle) -> Result<Vec<Provider>, String> {
    let path = get_providers_file_path(app)?;
    load_providers_from_path(&path)
}

#[tauri::command]
fn save_provider(app: tauri::AppHandle, provider: Provider) -> Result<(), String> {
    let path = get_providers_file_path(app.clone())?;
    let mut providers = load_providers(app)?;
    let previous_provider = providers
        .iter()
        .find(|item| item.id == provider.id)
        .cloned();
    let previous_active_provider = providers.iter().find(|item| item.is_active).cloned();
    let normalized_provider = provider.normalize();

    if normalized_provider.is_active {
        for item in providers.iter_mut() {
            item.is_active = item.id == normalized_provider.id;
        }
    }

    if let Some(pos) = providers
        .iter()
        .position(|item| item.id == normalized_provider.id)
    {
        providers[pos] = normalized_provider.clone();
    } else {
        providers.push(normalized_provider.clone());
    }

    ensure_single_active_provider(&mut providers);
    write_providers_to_path(&path, &providers)?;

    if normalized_provider.is_active
        || previous_provider
            .as_ref()
            .map(|item| item.is_active)
            .unwrap_or(false)
    {
        if let Some(active_provider) = providers.iter().find(|item| item.is_active) {
            update_claude_settings(previous_active_provider.as_ref(), active_provider)?;
        } else {
            clear_managed_claude_settings(&providers)?;
        }
    }

    Ok(())
}

#[tauri::command]
fn delete_provider(app: tauri::AppHandle, id: String) -> Result<(), String> {
    let path = get_providers_file_path(app.clone())?;
    let mut providers = load_providers(app)?;
    let deleted_provider = providers.iter().find(|item| item.id == id).cloned();

    providers.retain(|provider| provider.id != id);
    ensure_single_active_provider(&mut providers);
    write_providers_to_path(&path, &providers)?;

    if deleted_provider
        .as_ref()
        .map(|provider| provider.is_active)
        .unwrap_or(false)
    {
        if let Some(active_provider) = providers.iter().find(|provider| provider.is_active) {
            update_claude_settings(deleted_provider.as_ref(), active_provider)?;
        } else {
            clear_managed_claude_settings(&providers)?;
        }
    }

    Ok(())
}

#[tauri::command]
fn switch_provider(app: tauri::AppHandle, id: String) -> Result<(), String> {
    let path = get_providers_file_path(app.clone())?;
    let mut providers = load_providers(app)?;
    let previous_active_provider = providers
        .iter()
        .find(|provider| provider.is_active)
        .cloned();
    let provider = providers
        .iter()
        .find(|provider| provider.id == id)
        .ok_or("Provider not found")?
        .clone()
        .normalize();

    for item in &mut providers {
        item.is_active = item.id == id;
    }

    write_providers_to_path(&path, &providers)?;
    update_claude_settings(previous_active_provider.as_ref(), &provider)
}

#[tauri::command]
fn load_templates(app: tauri::AppHandle) -> Result<Vec<Template>, String> {
    let path = get_templates_file_path(app)?;
    if !path.exists() {
        return Ok(Vec::new());
    }

    let content =
        fs::read_to_string(&path).map_err(|e| format!("Failed to read templates file: {}", e))?;
    let templates: Vec<Template> = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse templates file: {}", e))?;
    Ok(templates)
}

#[tauri::command]
fn save_template(app: tauri::AppHandle, template: Template) -> Result<(), String> {
    let path = get_templates_file_path(app.clone())?;
    let mut templates = load_templates(app)?;

    if let Some(pos) = templates.iter().position(|item| item.id == template.id) {
        templates[pos] = template;
    } else {
        templates.push(template);
    }

    let content = serde_json::to_string_pretty(&templates)
        .map_err(|e| format!("Failed to serialize templates: {}", e))?;
    fs::write(&path, content).map_err(|e| format!("Failed to write templates file: {}", e))?;
    Ok(())
}

#[tauri::command]
fn delete_template(app: tauri::AppHandle, id: String) -> Result<(), String> {
    let path = get_templates_file_path(app.clone())?;
    let mut templates = load_templates(app)?;

    templates.retain(|template| template.id != id);

    let content = serde_json::to_string_pretty(&templates)
        .map_err(|e| format!("Failed to serialize templates: {}", e))?;
    fs::write(&path, content).map_err(|e| format!("Failed to write templates file: {}", e))?;
    Ok(())
}

#[tauri::command]
fn clear_all_providers(app: tauri::AppHandle) -> Result<(), String> {
    let path = get_providers_file_path(app.clone())?;
    let providers = load_providers(app)?;
    let empty: Vec<Provider> = Vec::new();

    write_providers_to_path(&path, &empty)?;
    clear_managed_claude_settings(&providers)
}

#[tauri::command]
fn export_providers_to_file(app: tauri::AppHandle, path: String) -> Result<(), String> {
    let providers = load_providers(app)?;
    let payload = ProviderTransferFile {
        version: PROVIDER_TRANSFER_VERSION,
        providers,
    };
    let content = serde_json::to_string_pretty(&payload)
        .map_err(|e| format!("Failed to serialize provider export: {}", e))?;
    fs::write(path, content).map_err(|e| format!("Failed to export providers: {}", e))?;
    Ok(())
}

#[tauri::command]
fn import_providers_from_file(
    app: tauri::AppHandle,
    payload: serde_json::Value,
) -> Result<usize, String> {
    let path = payload
        .get("path")
        .and_then(|value| value.as_str())
        .ok_or("Import payload.path must be a string")?;
    let replace_existing = payload
        .get("replaceExisting")
        .or_else(|| payload.get("replace_existing"))
        .and_then(|value| value.as_bool())
        .unwrap_or(false);

    let import_content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read import file: {}", e))?;
    let imported_providers = parse_providers_content(&import_content)?;
    let providers_path = get_providers_file_path(app.clone())?;

    let previous_providers = load_providers(app.clone())?;
    let previous_active_provider = previous_providers
        .iter()
        .find(|provider| provider.is_active)
        .cloned();

    let mut merged_providers = if replace_existing {
        imported_providers
    } else {
        let mut providers = previous_providers.clone();
        for provider in imported_providers {
            if let Some(pos) = providers.iter().position(|item| item.id == provider.id) {
                providers[pos] = provider;
            } else {
                providers.push(provider);
            }
        }
        providers
    };

    ensure_single_active_provider(&mut merged_providers);
    write_providers_to_path(&providers_path, &merged_providers)?;

    if let Some(active_provider) = merged_providers.iter().find(|provider| provider.is_active) {
        update_claude_settings(previous_active_provider.as_ref(), active_provider)?;
    } else if previous_active_provider.is_some() {
        clear_managed_claude_settings(&merged_providers)?;
    }

    Ok(merged_providers.len())
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
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            load_providers,
            save_provider,
            delete_provider,
            switch_provider,
            load_templates,
            save_template,
            delete_template,
            clear_all_providers,
            export_providers_to_file,
            import_providers_from_file,
            get_claude_settings_path_cmd,
        ])
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
