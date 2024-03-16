use crate::entity::record_item;
use anyhow::Result;
use chrono::prelude::*;
use reqwest::blocking::{Client, Response};
use ring::hmac;
use serde_json::Value;

pub struct TencentUtils {
    secret_id: String,
    secret_key: String,
}

impl TencentUtils {
    pub fn new(secret_id: String, secret_key: String) -> Self {
        Self {
            secret_id,
            secret_key,
        }
    }

    pub fn update_record(
        &self,
        domain: &str,
        sub_domain: &str,
        value: &str,
        record_id: i64,
    ) -> Result<()> {
        let action = "ModifyRecord";
        let body = format!(
            r#"{{"Domain": "{}", "SubDomain": "{}", "RecordType": "A", "RecordLine": "默认", "Value": "{}", "RecordId": "{}", "Remark": "ddns"}}"#,
            domain, sub_domain, value, record_id
        );
        do_request(action, &body, &self.secret_key, &self.secret_id)?;
        Ok(())
    }

    pub fn delete_record(&self, domain: &str, record_id: i64) -> Result<()> {
        let action = "DeleteRecord";
        let body = format!(r#"{{"Domain": "{}", "RecordId": "{}"}}"#, domain, record_id);
        do_request(action, &body, &self.secret_key, &self.secret_id)?;
        Ok(())
    }

    pub fn create_record(&self, domain: &str, sub_domain: &str, value: &str) -> Result<()> {
        let action = "CreateRecord";
        let body = format!(
            r#"{{"Domain": "{}", "SubDomain": "{}", "RecordType": "A", "RecordLine": "默认", "Value": "{}", "Remark": "ddns"}}"#,
            domain, sub_domain, value
        );
        do_request(action, &body, &self.secret_key, &self.secret_id)?;
        Ok(())
    }

    pub fn describe_record_list(&self, domain: &str) -> Result<Vec<record_item::RecordItem>> {
        let action = "DescribeRecordList";
        let body = format!(r#"{{"Domain": "{}"}}"#, domain);
        let result: Value = do_request(action, &body, &self.secret_key, &self.secret_id)?.json()?;
        let ret_array = result["Response"]["RecordList"].clone();
        Ok(serde_json::from_value(ret_array)?)
    }
}

fn do_request(action: &str, body: &str, secret_key: &str, secret_id: &str) -> Result<Response> {
    let host = "dnspod.tencentcloudapi.com";
    let endpoint = "https://dnspod.tencentcloudapi.com";
    let timestamp = Local::now().timestamp_millis();
    let auth = get_auth(timestamp, body, secret_key, secret_id);

    let client = Client::new();
    client
        .post(endpoint)
        .header("Host", host)
        .header("X-TC-Timestamp", timestamp / 1000)
        .header("X-TC-Version", "2021-03-23")
        .header("X-TC-Action", action)
        .header("X-TC-Region", "")
        .header("X-TC-Token", "")
        .header("X-TC-RequestClient", "SDK_JAVA_BAREBONE")
        .header("Authorization", auth)
        .header("Content-Type", "application/json; charset=utf-8")
        .body(body.to_string())
        .send()
        .map_err(|e| anyhow::anyhow!("Failed to send request: {}", e))
}

fn get_auth(timestamp: i64, body: &str, secret_key: &str, secret_id: &str) -> String {
    let hashed_request_payload = sha256_hex(body.as_bytes());

    let canonical_request = format!(
        "POST
/

content-type:application/json; charset=utf-8
host:dnspod.tencentcloudapi.com

content-type;host
{}",
        hashed_request_payload
    );

    let date = DateTime::from_timestamp_millis(timestamp).expect("Invalid timestamp");
    let date_str = date.format("%Y-%m-%d").to_string();
    let credential_scope = format!("{}/dnspod/tc3_request", date_str);
    let hashed_canonical_request = sha256_hex(canonical_request.as_bytes());
    let string_to_sign = format!(
        "TC3-HMAC-SHA256
{}
{}
{}",
        timestamp / 1000,
        credential_scope,
        hashed_canonical_request
    );

    let secret_date = hmac256(format!("TC3{}", secret_key).as_bytes(), &date_str);
    let secret_service = hmac256(&secret_date, "dnspod");
    let secret_signing = hmac256(&secret_service, "tc3_request");
    let signature = print_hex_binary(&hmac256(&secret_signing, &string_to_sign)).to_lowercase();

    format!(
        "TC3-HMAC-SHA256 Credential={}/{}, SignedHeaders=content-type;host, Signature={}",
        secret_id, credential_scope, signature
    )
}

fn hmac256(key: &[u8], msg: &str) -> Vec<u8> {
    let key = hmac::Key::new(hmac::HMAC_SHA256, key);
    let msg_bytes = msg.as_bytes();
    hmac::sign(&key, msg_bytes).as_ref().to_vec()
}

fn sha256_hex(data: &[u8]) -> String {
    let digest = ring::digest::digest(&ring::digest::SHA256, data);
    print_hex_binary(digest.as_ref())
}

fn print_hex_binary(data: &[u8]) -> String {
    data.iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hmac256() {
        let key = "TC3Bx7v3jJt9R5zv".as_bytes();
        let msg = "2019-01-01";
        let result = hmac256(key, msg);
        println!("{:?}", print_hex_binary(&result));

        println!("{:?}", sha256_hex("Hello World".as_bytes()));

        let timestamp = Local::now().timestamp_millis();

        println!("{:?}", timestamp);

        let date = DateTime::from_timestamp_millis(timestamp).unwrap();

        let date_str = date.format("%Y-%m-%d %H:%M:%S").to_string();

        println!("{:?}", date_str);
    }
}
