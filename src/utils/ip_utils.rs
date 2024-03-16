use anyhow::{anyhow, Result};
use reqwest::blocking::get;

pub fn get_ip() -> Result<String> {
    let result = get("http://checkip.amazonaws.com")
        .map_err(|_| anyhow!("IP网络请求失败"))?
        .text()
        .map_err(|_| anyhow!("IP结果解析失败"))?
        .trim().to_string();
    Ok(result)
}
