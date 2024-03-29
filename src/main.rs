use anyhow::{anyhow, Result};
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;
use utils::{file_utils, tencent_utils::TencentUtils};

#[macro_use]
extern crate lazy_static;

mod entity;
mod utils;

fn main() {

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut origin_config = file_utils::load_config().unwrap();

    let ddns_config = origin_config.clone();

    let ip = match check_cache(ddns_config.force_update) {
        Ok(ip) => ip,
        Err(e) => {
            info!("{}", e);
            return;
        }
    };

    let tencent_util = TencentUtils::new(ddns_config.secret_id, ddns_config.secret_key);

    let record_items = tencent_util
        .describe_record_list(&ddns_config.domain)
        .unwrap();

    let sub_domains = ddns_config.sub_domain;

    let mut remote_domains = vec![];

    record_items.iter().for_each(|item| {
        remote_domains.push(item.name.clone());

        if "ddns" == item.remark {
            if sub_domains.contains(&item.name) {
                if ip != item.value {
                    tencent_util
                        .update_record(&ddns_config.domain, &item.name, &ip, item.record_id)
                        .unwrap();
                    info!("Update record: {} to {}", item.name, ip);
                }
                else {
                    info!("Skip update record: {} to {}", item.name, ip);
                }
            } else {
                tencent_util
                    .delete_record(&ddns_config.domain, item.record_id)
                    .unwrap();
                info!("Delete record: {}", item.name);
            }
        }
    });

    sub_domains.iter().for_each(|sub_domain| {
        if !remote_domains.contains(sub_domain) {
            tencent_util
                .create_record(&ddns_config.domain, sub_domain, &ip)
                .unwrap();
            info!("Create record: {} to {}", sub_domain, ip);
        }
    });

    file_utils::save_cache(&ip).unwrap();

    origin_config.force_update = false;

    file_utils::save_config(&origin_config).unwrap();
}

fn check_cache(force_update: bool) -> Result<String> {
    let cache = file_utils::load_cache()?;

    let ip = utils::ip_utils::get_ip()?;

    if force_update || (ip != cache) {
        Ok(ip)
    }
    else {
        Err(anyhow!("SKIP: Cache IP not changed, skip update"))
    }
}
