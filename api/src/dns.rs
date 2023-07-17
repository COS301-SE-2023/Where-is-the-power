use log::debug;
use serde::Serialize;
use std::env;
use std::net::Ipv4Addr;
use std::str::FromStr;

#[derive(Debug, Serialize)]
pub struct DnsRecord {
    pub content: Ipv4Addr,
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub proxied: Option<bool>,
}

pub async fn update_dns() -> Result<(), reqwest::Error> {
    if let Ok(record) = env::var("UPDATE_DNS") {
        let api_key = env::var("CLOUDFLARE_API_KEY")
            .expect("CLOUDFLARE_API_KEY environment variable not set");
        let zone_id = env::var("CLOUDFLARE_ZONE_ID")
            .expect("CLOUDFLARE_ZONE_ID environment variable not set");
        let auth_email = env::var("CLOUDFLARE_AUTH_EMAIL")
            .expect("CLOUDFLARE_AUTH_EMAIL environment variable not set");
        let record_name = env::var("UPDATE_DNS_RECORD_NAME")
            .expect("UPDATE_DNS_RECORD_NAME environment variable not set");

        let client = reqwest::Client::new();
        let ip = Ipv4Addr::from_str(
            client
                .get("https://icanhazip.com")
                .send()
                .await?
                .text()
                .await?
                .trim(),
        )
        .expect("Couldn't fetch public IP");

        let dns_record = DnsRecord {
            content: ip,
            name: record_name,
            ty: "A".to_string(),
            proxied: Some(false),
        };

        let res = client
            .put(format!(
                "https://api.cloudflare.com/client/v4/zones/{zone_id}/dns_records/{record}"
            ))
            .header("X-Auth-Email", auth_email)
            .header("X-Auth-Key", &api_key)
            .json(&dns_record)
            .send()
            .await?;

        debug!("DNS update response: {res:?}");
        debug!("Response body: {}", res.text().await?);
    }
    Ok(())
}
