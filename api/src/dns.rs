use log::{debug, warn};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct DnsSecret {
    pub cloudflare_api_key: String,
    pub cloudflare_auth_email: String,
    pub cloudflare_zone_id: String,
    pub dns_record_name: String,
    pub dns_record_id: String,
}

pub async fn update_dns() -> Result<(), reqwest::Error> {
    if let Ok(secret) = env::var("DNS_SECRETS") {
        let secret: DnsSecret =
            serde_json::from_str::<DnsSecret>(&secret).expect("Couldn't parse DNS_SECRETS env var");

        let zone_id = secret.cloudflare_zone_id;
        let api_key = secret.cloudflare_api_key;
        let auth_email = secret.cloudflare_auth_email;
        let record_id = secret.dns_record_id;
        let record_name = secret.dns_record_name;

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
                "https://api.cloudflare.com/client/v4/zones/{zone_id}/dns_records/{record_id}"
            ))
            .header("X-Auth-Email", auth_email)
            .header("X-Auth-Key", &api_key)
            .json(&dns_record)
            .send()
            .await?;

        debug!("DNS update response: {res:?}");
        debug!("Response body: {}", res.text().await?);
    } else {
        warn!("DNS_SECRETS environment variable not set. Won't attempt to update DNS records");
    }
    Ok(())
}
