use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::{Result, WechatError};
use crate::storage::state_dir::resolve_state_dir;

const SPACES_DIR_NAME: &str = "wechat-agent";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceConfig {
    pub name: String,
    pub agent: String,
    pub account_id: Option<String>,
    #[serde(default)]
    pub user_bindings: BTreeMap<String, String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct SpaceSummary {
    pub name: String,
    pub agent: String,
    pub account_id: Option<String>,
    pub binding_count: usize,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SpaceInspect {
    pub name: String,
    pub agent: String,
    pub account_id: Option<String>,
    pub user_bindings: BTreeMap<String, String>,
    pub created_at: String,
    pub updated_at: String,
    pub space_dir: String,
    pub log_file: String,
    pub pid_file: String,
    pub pid: Option<u32>,
}

pub fn normalize_space_name(raw: &str) -> String {
    raw.trim()
        .to_lowercase()
        .chars()
        .map(|ch| match ch {
            'a'..='z' | '0'..='9' | '-' | '_' => ch,
            _ => '-',
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

pub fn create_space(name: &str, agent: &str, account_id: Option<String>) -> Result<SpaceConfig> {
    let normalized = normalize_space_name(name);
    if normalized.is_empty() {
        return Err(WechatError::Api("space name is required".to_string()));
    }

    let path = space_config_path(&normalized);
    if path.exists() {
        return Err(WechatError::Api(format!("space already exists: {normalized}")));
    }

    fs::create_dir_all(space_dir(&normalized))?;
    let cfg = SpaceConfig {
        name: normalized.clone(),
        agent: agent.trim().to_lowercase(),
        account_id,
        user_bindings: BTreeMap::new(),
        created_at: unix_ts_string(),
        updated_at: unix_ts_string(),
    };
    save_space(&cfg)?;
    Ok(cfg)
}

pub fn save_space(space: &SpaceConfig) -> Result<()> {
    fs::create_dir_all(space_dir(&space.name))?;
    fs::write(space_config_path(&space.name), serde_json::to_vec_pretty(space)?)?;
    Ok(())
}

pub fn load_space(name: &str) -> Result<SpaceConfig> {
    let normalized = normalize_space_name(name);
    let raw = fs::read_to_string(space_config_path(&normalized))
        .map_err(|_| WechatError::Api(format!("space not found: {normalized}")))?;
    Ok(serde_json::from_str(&raw)?)
}

pub fn inspect_space(name: &str) -> Result<SpaceInspect> {
    let space = load_space(name)?;
    Ok(SpaceInspect {
        name: space.name.clone(),
        agent: space.agent.clone(),
        account_id: space.account_id.clone(),
        user_bindings: space.user_bindings.clone(),
        created_at: space.created_at.clone(),
        updated_at: space.updated_at.clone(),
        space_dir: space_dir(&space.name).to_string_lossy().to_string(),
        log_file: space_log_path(&space.name).to_string_lossy().to_string(),
        pid_file: space_pid_path(&space.name).to_string_lossy().to_string(),
        pid: read_space_pid(&space.name),
    })
}

pub fn list_spaces() -> Result<Vec<SpaceSummary>> {
    let dir = spaces_root();
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut spaces = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if let Ok(space) = load_space(&name) {
            spaces.push(SpaceSummary {
                name: space.name,
                agent: space.agent,
                account_id: space.account_id,
                binding_count: space.user_bindings.len(),
                updated_at: space.updated_at,
            });
        }
    }
    spaces.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(spaces)
}

pub fn delete_space(name: &str) -> Result<()> {
    let normalized = normalize_space_name(name);
    let dir = space_dir(&normalized);
    if !dir.exists() {
        return Err(WechatError::Api(format!("space not found: {normalized}")));
    }
    fs::remove_dir_all(dir)?;
    Ok(())
}

pub fn set_space_account(name: &str, account_id: Option<String>) -> Result<SpaceConfig> {
    let mut space = load_space(name)?;
    space.account_id = account_id;
    space.updated_at = unix_ts_string();
    save_space(&space)?;
    Ok(space)
}

pub fn switch_space_agent(name: &str, agent: &str) -> Result<SpaceConfig> {
    let mut space = load_space(name)?;
    space.agent = agent.trim().to_lowercase();
    space.updated_at = unix_ts_string();
    save_space(&space)?;
    Ok(space)
}

pub fn set_user_binding(name: &str, user_id: &str, agent: &str) -> Result<SpaceConfig> {
    let mut space = load_space(name)?;
    let user = user_id.trim().to_string();
    if user.is_empty() {
        return Err(WechatError::Api("user id is required".to_string()));
    }
    space.user_bindings.insert(user, agent.trim().to_lowercase());
    space.updated_at = unix_ts_string();
    save_space(&space)?;
    Ok(space)
}

pub fn remove_user_binding(name: &str, user_id: &str) -> Result<SpaceConfig> {
    let mut space = load_space(name)?;
    space.user_bindings.remove(user_id.trim());
    space.updated_at = unix_ts_string();
    save_space(&space)?;
    Ok(space)
}

pub fn available_agents() -> &'static [&'static str] {
    &["claude", "codex", "openclaw", "openai", "anthropic", "echo"]
}

pub fn ensure_space_runtime_dirs(name: &str) -> Result<()> {
    fs::create_dir_all(space_dir(name).join("logs"))?;
    Ok(())
}

pub fn space_root_dir(name: &str) -> PathBuf {
    space_dir(&normalize_space_name(name))
}

pub fn space_log_path(name: &str) -> PathBuf {
    space_root_dir(name).join("logs").join("space.log")
}

pub fn space_pid_path(name: &str) -> PathBuf {
    space_root_dir(name).join("run.pid")
}

pub fn read_space_pid(name: &str) -> Option<u32> {
    let raw = fs::read_to_string(space_pid_path(name)).ok()?;
    raw.trim().parse::<u32>().ok()
}

pub fn write_space_pid(name: &str, pid: u32) -> Result<()> {
    ensure_space_runtime_dirs(name)?;
    fs::write(space_pid_path(name), pid.to_string())?;
    Ok(())
}

pub fn clear_space_pid(name: &str) -> Result<()> {
    let path = space_pid_path(name);
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

fn spaces_root() -> PathBuf {
    resolve_state_dir().join(SPACES_DIR_NAME).join("spaces")
}

fn space_dir(name: &str) -> PathBuf {
    spaces_root().join(name)
}

fn space_config_path(name: &str) -> PathBuf {
    space_dir(name).join("space.json")
}

fn unix_ts_string() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|v| v.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}
