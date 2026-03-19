use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Project {
    id: String,
    name: String,
    path: String,
    #[serde(rename = "providerId")]
    provider_id: String,
    #[serde(default)]
    model: String,
    #[serde(rename = "terminalTool")]
    #[serde(default)]
    terminal_tool: String,
    #[serde(rename = "launchCommandTemplate")]
    #[serde(default)]
    launch_command_template: String,
    #[serde(rename = "terminalOpenCommandTemplate")]
    #[serde(default)]
    terminal_open_command_template: String,
    #[serde(rename = "createdAt")]
    created_at: i64,
    #[serde(rename = "updatedAt")]
    updated_at: i64,
}

#[derive(Debug, Clone, Serialize)]
struct TerminalToolPreset {
    id: String,
    name: String,
    description: String,
    #[serde(rename = "defaultCommandTemplate")]
    default_command_template: String,
    #[serde(rename = "defaultOpenCommandTemplate")]
    default_open_command_template: String,
    #[serde(rename = "requiresOpenCommandTemplate")]
    requires_open_command_template: bool,
}

#[derive(Debug, Clone, Serialize)]
struct LaunchProjectResult {
    command: String,
    #[serde(rename = "terminalTool")]
    terminal_tool: String,
    #[serde(rename = "providerName")]
    provider_name: String,
    #[serde(rename = "projectName")]
    project_name: String,
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

#[derive(Clone, Copy)]
enum ShellKind {
    Posix,
    PowerShell,
    Cmd,
}

fn default_terminal_tool_id() -> &'static str {
    if cfg!(target_os = "macos") {
        "terminal"
    } else if cfg!(target_os = "windows") {
        "powershell"
    } else {
        "gnome-terminal"
    }
}

fn terminal_requires_open_command_template(tool_id: &str) -> bool {
    tool_id == "custom"
}

#[cfg(target_os = "macos")]
fn mac_app_installed(app_name: &str) -> bool {
    Command::new("open")
        .args(["-Ra", app_name])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

#[cfg(target_os = "windows")]
fn command_exists(command: &str) -> bool {
    Command::new("cmd")
        .args(["/C", &format!("where {command} >nul 2>nul")])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

#[cfg(all(unix, not(target_os = "macos")))]
fn command_exists(command: &str) -> bool {
    Command::new("sh")
        .args(["-lc", &format!("command -v {command} >/dev/null 2>&1")])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn terminal_tool_available(tool_id: &str) -> bool {
    if terminal_requires_open_command_template(tool_id) {
        return true;
    }

    #[cfg(target_os = "macos")]
    {
        return match tool_id {
            "terminal" => mac_app_installed("Terminal"),
            "iterm" => mac_app_installed("iTerm"),
            "ghostty" => mac_app_installed("Ghostty"),
            _ => false,
        };
    }

    #[cfg(target_os = "windows")]
    {
        return match tool_id {
            "powershell" => true,
            "cmd" => true,
            "windows-terminal" => command_exists("wt.exe"),
            _ => false,
        };
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        return match tool_id {
            "gnome-terminal" => command_exists("gnome-terminal"),
            "konsole" => command_exists("konsole"),
            "xterm" => command_exists("xterm"),
            _ => false,
        };
    }

    #[allow(unreachable_code)]
    false
}

fn default_launch_command_template(tool_id: &str) -> &'static str {
    match tool_id {
        "powershell" | "windows-terminal" => {
            "Set-Location -LiteralPath {projectPath}; claude --model {model} --dangerously-skip-permissions"
        }
        "cmd" => "cd /d {projectPath} && claude --model {model} --dangerously-skip-permissions",
        _ => "cd {projectPath} && claude --model {model} --dangerously-skip-permissions",
    }
}

fn default_open_command_template(tool_id: &str) -> &'static str {
    match tool_id {
        "custom" => {
            #[cfg(target_os = "macos")]
            {
                return "open -na Ghostty --args -e /bin/zsh -lc {command}";
            }
            #[cfg(target_os = "windows")]
            {
                return "wt.exe new-tab powershell.exe -NoExit -Command {command}";
            }
            #[cfg(all(unix, not(target_os = "macos")))]
            {
                return "xterm -hold -e bash -lc {command}";
            }
            #[allow(unreachable_code)]
            ""
        }
        _ => "",
    }
}

fn terminal_tool_presets() -> Vec<TerminalToolPreset> {
    #[cfg(target_os = "macos")]
    {
        return vec![
            TerminalToolPreset {
                id: "terminal".to_string(),
                name: "Terminal".to_string(),
                description: "macOS Terminal.app".to_string(),
                default_command_template: default_launch_command_template("terminal").to_string(),
                default_open_command_template: default_open_command_template("terminal")
                    .to_string(),
                requires_open_command_template: terminal_requires_open_command_template("terminal"),
            },
            TerminalToolPreset {
                id: "iterm".to_string(),
                name: "iTerm".to_string(),
                description: "iTerm2".to_string(),
                default_command_template: default_launch_command_template("iterm").to_string(),
                default_open_command_template: default_open_command_template("iterm")
                    .to_string(),
                requires_open_command_template: terminal_requires_open_command_template("iterm"),
            },
            TerminalToolPreset {
                id: "ghostty".to_string(),
                name: "Ghostty".to_string(),
                description: "Ghostty.app".to_string(),
                default_command_template: default_launch_command_template("ghostty").to_string(),
                default_open_command_template: default_open_command_template("ghostty")
                    .to_string(),
                requires_open_command_template: terminal_requires_open_command_template("ghostty"),
            },
            TerminalToolPreset {
                id: "custom".to_string(),
                name: "自定义".to_string(),
                description: "自定义终端打开命令".to_string(),
                default_command_template: default_launch_command_template("custom").to_string(),
                default_open_command_template: default_open_command_template("custom")
                    .to_string(),
                requires_open_command_template: terminal_requires_open_command_template("custom"),
            },
        ]
        .into_iter()
        .filter(|preset| terminal_tool_available(&preset.id))
        .collect();
    }

    #[cfg(target_os = "windows")]
    {
        return vec![
            TerminalToolPreset {
                id: "powershell".to_string(),
                name: "PowerShell".to_string(),
                description: "Windows PowerShell".to_string(),
                default_command_template: default_launch_command_template("powershell").to_string(),
                default_open_command_template: default_open_command_template("powershell")
                    .to_string(),
                requires_open_command_template: terminal_requires_open_command_template(
                    "powershell",
                ),
            },
            TerminalToolPreset {
                id: "windows-terminal".to_string(),
                name: "Windows Terminal".to_string(),
                description: "wt.exe".to_string(),
                default_command_template: default_launch_command_template("windows-terminal")
                    .to_string(),
                default_open_command_template: default_open_command_template("windows-terminal")
                    .to_string(),
                requires_open_command_template: terminal_requires_open_command_template(
                    "windows-terminal",
                ),
            },
            TerminalToolPreset {
                id: "cmd".to_string(),
                name: "Command Prompt".to_string(),
                description: "cmd.exe".to_string(),
                default_command_template: default_launch_command_template("cmd").to_string(),
                default_open_command_template: default_open_command_template("cmd").to_string(),
                requires_open_command_template: terminal_requires_open_command_template("cmd"),
            },
            TerminalToolPreset {
                id: "custom".to_string(),
                name: "Custom".to_string(),
                description: "Custom terminal open command".to_string(),
                default_command_template: default_launch_command_template("custom").to_string(),
                default_open_command_template: default_open_command_template("custom")
                    .to_string(),
                requires_open_command_template: terminal_requires_open_command_template("custom"),
            },
        ]
        .into_iter()
        .filter(|preset| terminal_tool_available(&preset.id))
        .collect();
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        return vec![
            TerminalToolPreset {
                id: "gnome-terminal".to_string(),
                name: "GNOME Terminal".to_string(),
                description: "gnome-terminal".to_string(),
                default_command_template: default_launch_command_template("gnome-terminal")
                    .to_string(),
                default_open_command_template: default_open_command_template("gnome-terminal")
                    .to_string(),
                requires_open_command_template: terminal_requires_open_command_template(
                    "gnome-terminal",
                ),
            },
            TerminalToolPreset {
                id: "konsole".to_string(),
                name: "Konsole".to_string(),
                description: "KDE Konsole".to_string(),
                default_command_template: default_launch_command_template("konsole").to_string(),
                default_open_command_template: default_open_command_template("konsole")
                    .to_string(),
                requires_open_command_template: terminal_requires_open_command_template("konsole"),
            },
            TerminalToolPreset {
                id: "xterm".to_string(),
                name: "XTerm".to_string(),
                description: "xterm".to_string(),
                default_command_template: default_launch_command_template("xterm").to_string(),
                default_open_command_template: default_open_command_template("xterm").to_string(),
                requires_open_command_template: terminal_requires_open_command_template("xterm"),
            },
            TerminalToolPreset {
                id: "custom".to_string(),
                name: "Custom".to_string(),
                description: "Custom terminal open command".to_string(),
                default_command_template: default_launch_command_template("custom").to_string(),
                default_open_command_template: default_open_command_template("custom")
                    .to_string(),
                requires_open_command_template: terminal_requires_open_command_template("custom"),
            },
        ]
        .into_iter()
        .filter(|preset| terminal_tool_available(&preset.id))
        .collect();
    }

    #[allow(unreachable_code)]
    Vec::new()
}

fn shell_kind_for_tool(tool_id: &str) -> Result<ShellKind, String> {
    match tool_id {
        "powershell" | "windows-terminal" => Ok(ShellKind::PowerShell),
        "cmd" => Ok(ShellKind::Cmd),
        "terminal" | "iterm" | "ghostty" | "gnome-terminal" | "konsole" | "xterm" => {
            Ok(ShellKind::Posix)
        }
        "custom" => {
            if cfg!(target_os = "windows") {
                Ok(ShellKind::PowerShell)
            } else {
                Ok(ShellKind::Posix)
            }
        }
        _ => Err(format!("Unsupported terminal tool: {}", tool_id)),
    }
}

fn escape_posix(value: &str) -> String {
    if value.is_empty() {
        "''".to_string()
    } else {
        format!("'{}'", value.replace('\'', "'\"'\"'"))
    }
}

fn escape_powershell(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

fn escape_cmd(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\"\""))
}

fn render_launch_command(
    template: &str,
    project_path: &str,
    model: &str,
    shell_kind: ShellKind,
) -> String {
    let escape = |value: &str| match shell_kind {
        ShellKind::Posix => escape_posix(value),
        ShellKind::PowerShell => escape_powershell(value),
        ShellKind::Cmd => escape_cmd(value),
    };

    template
        .replace("{projectPath}", &escape(project_path))
        .replace("{model}", &escape(model))
}

fn render_open_command(template: &str, command: &str, shell_kind: ShellKind) -> String {
    let escaped_command = match shell_kind {
        ShellKind::Posix => escape_posix(command),
        ShellKind::PowerShell => escape_powershell(command),
        ShellKind::Cmd => escape_cmd(command),
    };

    template.replace("{command}", &escaped_command)
}

fn claude_installed() -> bool {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "where claude >nul 2>nul"])
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    } else {
        Command::new("sh")
            .args(["-lc", "command -v claude >/dev/null 2>&1"])
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }
}

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

fn json_value_to_env_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(inner) => inner.clone(),
        serde_json::Value::Null => String::new(),
        other => other.to_string(),
    }
}

fn provider_env_pairs(provider: &Provider) -> Vec<(String, String)> {
    let anthropic_model = provider
        .anthropic_model
        .as_deref()
        .filter(|model| !model.trim().is_empty())
        .unwrap_or(&provider.models.default)
        .to_string();
    let anthropic_small_fast_model = provider
        .anthropic_small_fast_model
        .as_deref()
        .filter(|model| !model.trim().is_empty())
        .unwrap_or(&provider.models.small_fast)
        .to_string();

    let mut pairs = vec![
        ("ANTHROPIC_BASE_URL".to_string(), provider.base_url.clone()),
        ("ANTHROPIC_AUTH_TOKEN".to_string(), provider.api_key.clone()),
        ("ANTHROPIC_MODEL".to_string(), anthropic_model),
        (
            "ANTHROPIC_SMALL_FAST_MODEL".to_string(),
            anthropic_small_fast_model,
        ),
        (
            "ANTHROPIC_DEFAULT_SONNET_MODEL".to_string(),
            provider.models.sonnet.clone(),
        ),
        (
            "ANTHROPIC_DEFAULT_OPUS_MODEL".to_string(),
            provider.models.opus.clone(),
        ),
        (
            "ANTHROPIC_DEFAULT_HAIKU_MODEL".to_string(),
            provider.models.haiku.clone(),
        ),
    ];

    for (key, value) in provider.custom_env.iter() {
        pairs.push((key.clone(), json_value_to_env_string(value)));
    }

    pairs
}

fn prepend_provider_env_to_command(
    command: &str,
    provider: &Provider,
    shell_kind: ShellKind,
) -> String {
    let prefix = provider_env_pairs(provider)
        .into_iter()
        .map(|(key, value)| match shell_kind {
            ShellKind::Posix => format!("export {key}={}", escape_posix(&value)),
            ShellKind::PowerShell => format!("$env:{key}={}", escape_powershell(&value)),
            ShellKind::Cmd => format!("set \"{key}={value}\""),
        })
        .collect::<Vec<_>>();

    let separator = match shell_kind {
        ShellKind::Posix | ShellKind::PowerShell => "; ",
        ShellKind::Cmd => " && ",
    };

    format!("{}{}{}", prefix.join(separator), separator, command)
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

impl Project {
    fn normalize(mut self, providers: &[Provider]) -> Self {
        let available_presets = terminal_tool_presets();
        if self.terminal_tool.trim().is_empty() {
            self.terminal_tool = available_presets
                .first()
                .map(|preset| preset.id.clone())
                .unwrap_or_else(|| default_terminal_tool_id().to_string());
        } else if !available_presets
            .iter()
            .any(|preset| preset.id == self.terminal_tool)
        {
            self.terminal_tool = available_presets
                .first()
                .map(|preset| preset.id.clone())
                .unwrap_or_else(|| default_terminal_tool_id().to_string());
        }
        if self.launch_command_template.trim().is_empty() {
            self.launch_command_template =
                default_launch_command_template(&self.terminal_tool).to_string();
        }
        if self.model.trim().is_empty() {
            if let Some(provider) = providers.iter().find(|item| item.id == self.provider_id) {
                self.model = provider.models.default.clone();
            }
        }
        if terminal_requires_open_command_template(&self.terminal_tool)
            && self.terminal_open_command_template.trim().is_empty()
        {
            self.terminal_open_command_template =
                default_open_command_template(&self.terminal_tool).to_string();
        }
        self
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

fn get_projects_file_path(app: tauri::AppHandle) -> Result<PathBuf, String> {
    let mut path = get_app_data_dir(app)?;
    fs::create_dir_all(&path).map_err(|e| format!("Failed to create app data dir: {}", e))?;
    path.push("projects.json");
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

fn parse_projects_content(content: &str, providers: &[Provider]) -> Result<Vec<Project>, String> {
    let projects: Vec<Project> = serde_json::from_str(content)
        .map_err(|e| format!("Failed to parse projects file: {}", e))?;
    Ok(projects
        .into_iter()
        .map(|project| project.normalize(providers))
        .collect())
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

fn load_projects_from_path(path: &Path, providers: &[Provider]) -> Result<Vec<Project>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read projects file: {}", e))?;

    if content.trim().is_empty() {
        return Ok(Vec::new());
    }

    let projects = parse_projects_content(&content, providers)?;
    let normalized_content = serde_json::to_string_pretty(&projects)
        .map_err(|e| format!("Failed to serialize projects: {}", e))?;
    if content.trim() != normalized_content {
        fs::write(path, normalized_content)
            .map_err(|e| format!("Failed to write migrated projects: {}", e))?;
    }

    Ok(projects)
}

fn write_providers_to_path(path: &Path, providers: &[Provider]) -> Result<(), String> {
    let content = serde_json::to_string_pretty(providers)
        .map_err(|e| format!("Failed to serialize providers: {}", e))?;
    fs::write(path, content).map_err(|e| format!("Failed to write providers file: {}", e))
}

fn write_projects_to_path(path: &Path, projects: &[Project]) -> Result<(), String> {
    let content = serde_json::to_string_pretty(projects)
        .map_err(|e| format!("Failed to serialize projects: {}", e))?;
    fs::write(path, content).map_err(|e| format!("Failed to write projects file: {}", e))
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

fn activate_provider(app: tauri::AppHandle, id: &str) -> Result<Provider, String> {
    let path = get_providers_file_path(app.clone())?;
    let mut providers = load_providers(app)?;
    let previous_active_provider = providers.iter().find(|provider| provider.is_active).cloned();
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
    update_claude_settings(previous_active_provider.as_ref(), &provider)?;
    Ok(provider)
}

fn validate_project(project: &Project, providers: &[Provider]) -> Result<(), String> {
    if project.name.trim().is_empty() {
        return Err("Project name is required".to_string());
    }
    if project.path.trim().is_empty() {
        return Err("Project path is required".to_string());
    }
    if project.provider_id.trim().is_empty() {
        return Err("Provider is required".to_string());
    }
    if providers.iter().all(|provider| provider.id != project.provider_id) {
        return Err("Selected provider not found".to_string());
    }
    if project.model.trim().is_empty() {
        return Err("Model is required".to_string());
    }
    if project.terminal_tool.trim().is_empty() {
        return Err("Terminal tool is required".to_string());
    }
    if project.launch_command_template.trim().is_empty() {
        return Err("Launch command template is required".to_string());
    }
    if terminal_requires_open_command_template(&project.terminal_tool)
        && project.terminal_open_command_template.trim().is_empty()
    {
        return Err("Terminal open command template is required".to_string());
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn launch_command_in_terminal(
    tool_id: &str,
    command: &str,
    open_command_template: Option<&str>,
) -> Result<(), String> {
    if tool_id == "ghostty" {
        Command::new("open")
            .args(["-na", "Ghostty", "--args", "-e", "/bin/zsh", "-lc", command])
            .spawn()
            .map_err(|e| format!("Failed to open Ghostty: {}", e))?;
        return Ok(());
    }

    if tool_id == "custom" {
        let template = open_command_template
            .ok_or("Custom terminal open command template is required")?;
        let rendered = render_open_command(template, command, ShellKind::Posix);
        Command::new("sh")
            .args(["-lc", &rendered])
            .spawn()
            .map_err(|e| format!("Failed to open custom terminal: {}", e))?;
        return Ok(());
    }

    let script_lines = match tool_id {
        "terminal" => vec![
            "on run argv",
            "tell application id \"com.apple.Terminal\"",
            "activate",
            "do script (item 1 of argv)",
            "end tell",
            "end run",
        ],
        "iterm" => vec![
            "on run argv",
            "tell application id \"com.googlecode.iterm2\"",
            "activate",
            "set newWindow to create window with default profile",
            "tell current session of newWindow to write text (item 1 of argv)",
            "end tell",
            "end run",
        ],
        _ => return Err(format!("Unsupported terminal tool: {}", tool_id)),
    };

    let mut apple_script = Command::new("osascript");
    for line in script_lines {
        apple_script.arg("-e").arg(line);
    }
    let output = apple_script
        .arg(command)
        .output()
        .map_err(|e| format!("Failed to open terminal: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if stderr.is_empty() {
            return Err(format!("Failed to open terminal tool: {}", tool_id));
        }
        return Err(format!(
            "Failed to open terminal tool {}: {}",
            tool_id, stderr
        ));
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn launch_command_in_terminal(
    tool_id: &str,
    command: &str,
    open_command_template: Option<&str>,
) -> Result<(), String> {
    let mut child = match tool_id {
        "powershell" => Command::new("powershell.exe")
            .args(["-NoExit", "-Command", command])
            .spawn(),
        "windows-terminal" => Command::new("wt.exe")
            .args(["new-tab", "powershell.exe", "-NoExit", "-Command", command])
            .spawn(),
        "cmd" => Command::new("cmd.exe").args(["/K", command]).spawn(),
        "custom" => {
            let template = open_command_template
                .ok_or("Custom terminal open command template is required")?;
            let rendered = render_open_command(template, command, ShellKind::PowerShell);
            Command::new("powershell.exe")
                .args(["-NoProfile", "-Command", &rendered])
                .spawn()
        }
        _ => return Err(format!("Unsupported terminal tool: {}", tool_id)),
    }
    .map_err(|e| format!("Failed to open terminal: {}", e))?;

    let _ = child.id();
    Ok(())
}

#[cfg(all(unix, not(target_os = "macos")))]
fn launch_command_in_terminal(
    tool_id: &str,
    command: &str,
    open_command_template: Option<&str>,
) -> Result<(), String> {
    if tool_id == "custom" {
        let template = open_command_template
            .ok_or("Custom terminal open command template is required")?;
        let rendered = render_open_command(template, command, ShellKind::Posix);
        Command::new("sh")
            .args(["-lc", &rendered])
            .spawn()
            .map_err(|e| format!("Failed to open custom terminal: {}", e))?;
        return Ok(());
    }

    let hold_command = format!("{command}; exec bash");
    let mut child = match tool_id {
        "gnome-terminal" => Command::new("gnome-terminal")
            .args(["--", "bash", "-lc", &hold_command])
            .spawn(),
        "konsole" => Command::new("konsole")
            .args(["--noclose", "-e", "bash", "-lc", &hold_command])
            .spawn(),
        "xterm" => Command::new("xterm")
            .args(["-hold", "-e", "bash", "-lc", &hold_command])
            .spawn(),
        _ => return Err(format!("Unsupported terminal tool: {}", tool_id)),
    }
    .map_err(|e| format!("Failed to open terminal: {}", e))?;

    let _ = child.id();
    Ok(())
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
    activate_provider(app, &id).map(|_| ())
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

#[tauri::command]
fn load_projects(app: tauri::AppHandle) -> Result<Vec<Project>, String> {
    let providers = load_providers(app.clone())?;
    let path = get_projects_file_path(app)?;
    load_projects_from_path(&path, &providers)
}

#[tauri::command]
fn save_project(app: tauri::AppHandle, project: Project) -> Result<(), String> {
    let providers = load_providers(app.clone())?;
    let path = get_projects_file_path(app.clone())?;
    let mut projects = load_projects_from_path(&path, &providers)?;
    let normalized_project = project.normalize(&providers);

    validate_project(&normalized_project, &providers)?;

    if let Some(pos) = projects
        .iter()
        .position(|item| item.id == normalized_project.id)
    {
        projects[pos] = normalized_project;
    } else {
        projects.push(normalized_project);
    }

    write_projects_to_path(&path, &projects)
}

#[tauri::command]
fn delete_project(app: tauri::AppHandle, id: String) -> Result<(), String> {
    let providers = load_providers(app.clone())?;
    let path = get_projects_file_path(app)?;
    let mut projects = load_projects_from_path(&path, &providers)?;
    projects.retain(|project| project.id != id);
    write_projects_to_path(&path, &projects)
}

#[tauri::command]
fn get_terminal_tool_presets() -> Vec<TerminalToolPreset> {
    terminal_tool_presets()
}

#[tauri::command]
fn launch_project(app: tauri::AppHandle, id: String) -> Result<LaunchProjectResult, String> {
    let providers = load_providers(app.clone())?;
    let projects_path = get_projects_file_path(app.clone())?;
    let projects = load_projects_from_path(&projects_path, &providers)?;
    let project = projects
        .iter()
        .find(|project| project.id == id)
        .ok_or("Project not found")?
        .clone();

    validate_project(&project, &providers)?;

    if !Path::new(&project.path).exists() {
        return Err("Project path does not exist".to_string());
    }

    if !claude_installed() {
        return Err("请先安装 claude".to_string());
    }

    let provider = providers
        .iter()
        .find(|provider| provider.id == project.provider_id)
        .ok_or("Selected provider not found")?
        .clone();
    let shell_kind = shell_kind_for_tool(&project.terminal_tool)?;
    let launch_command = render_launch_command(
        &project.launch_command_template,
        &project.path,
        &project.model,
        shell_kind,
    );
    let command = prepend_provider_env_to_command(&launch_command, &provider, shell_kind);

    let open_command_template = if terminal_requires_open_command_template(&project.terminal_tool) {
        Some(project.terminal_open_command_template.as_str())
    } else {
        None
    };

    launch_command_in_terminal(&project.terminal_tool, &command, open_command_template)?;

    Ok(LaunchProjectResult {
        command,
        terminal_tool: project.terminal_tool,
        provider_name: provider.name,
        project_name: project.name,
    })
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
            load_projects,
            save_project,
            delete_project,
            get_terminal_tool_presets,
            launch_project,
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
