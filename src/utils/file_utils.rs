use std::fs;

use anyhow::{anyhow, Ok, Result};
use clap::Parser;
use std::path::Path;

use crate::entity::{constans::CACHE_PATH, ddns_config::DDNSConfig};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub config_path: Option<String>,
    pub cache_path: Option<String>,
}

/**
 * 处理路径
 * 如果路径为空，则使用默认路径
 */
pub fn handle_path() -> Args {
    let mut args = Args::parse();
    if args.config_path.is_none() {
        if let Some(home_dir) = dirs::home_dir() {
            let config_path = home_dir.join(".ddns/.ddns_config");
            args.config_path = Some(config_path.to_string_lossy().to_string());
        }
    }
    if args.cache_path.is_none() {
        if let Some(home_dir) = dirs::home_dir() {
            let cache_path = home_dir.join(".ddns/.ddns_cache");
            args.cache_path = Some(cache_path.to_string_lossy().to_string());
        }
    }
    args
}

/**
 * 加载缓存
 */
pub fn load_cache() -> Result<String> {
    let cache_path = Path::new(&*CACHE_PATH);
    if !cache_path.exists() {
        std::fs::create_dir_all(cache_path).map_err(|_| anyhow!("缓存文件创建失败"))?;
    }
    let cache = fs::read_to_string(cache_path).map_err(|_| anyhow!("缓存文件解析失败"))?;
    Ok(cache)
}

/**
 * 保存缓存
 */
pub fn save_cache(content: &str) -> Result<()> {
    let cache_path = Path::new(&*CACHE_PATH);
    fs::write(cache_path, content).map_err(|_| anyhow!("缓存文件写入失败"))?;
    Ok(())
}

/**
 * 加载配置文件
 */
pub fn load_config() -> Result<DDNSConfig> {
    let config_path = Path::new(&*CACHE_PATH);
    let content = fs::read_to_string(config_path).map_err(|_| anyhow!("配置文件读取失败"))?;
    let ddns_config = serde_json::from_str(&content).map_err(|_| anyhow!("配置文件解析失败"))?;
    Ok(ddns_config)
}

/**
 * 保存配置文件
 */
pub fn save_config(ddns_config: &DDNSConfig) -> Result<()> {
    let config_path = Path::new(&*CACHE_PATH);
    let content = serde_json::to_string(&ddns_config).map_err(|_| anyhow!("配置文件序列化失败"))?;
    fs::write(config_path, content).map_err(|_| anyhow!("配置文件写入失败"))?;
    Ok(())
}