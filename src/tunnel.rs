use getrandom::getrandom;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use worker::*;
use crate::config::Config;
use crate::proxy::ProxyStream;

static PROXYIP_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^.+-\d+$").unwrap());

static PROXYKV_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([A-Z]{2})").unwrap());

pub async fn handle(
    req: Request,
    mut cx: RouteContext<Config>,
) -> Result<Response> {

    let mut proxyip =
        cx.param("proxyip").cloned().unwrap_or_default();

    if proxyip.is_empty() {

        if let Ok(url) = req.url() {

            if let Some(path) =
                url.path_segments().and_then(|mut x| x.next())
            {
                proxyip = path.to_string();
            }

        }

    }

    let proxyip_lower = proxyip.to_lowercase();

    match proxyip_lower.as_str() {

        "vmess" => {
            proxyip = "ID".to_string();
        }

        "vless" => {
            proxyip = "ID".to_string();
        }

        "trojan" => {
            proxyip = "ID".to_string();
        }

        "shadowsocks" => {
            proxyip = "ID".to_string();
        }

        _ => {}

    }
    
    proxyip = proxyip.to_uppercase();
    if PROXYKV_PATTERN.is_match(&proxyip) {
    
        let kv = cx.kv("SIREN")?;
    
        let mut proxy_kv_str = kv
            .get("proxy_kv")
            .text()
            .await?
            .unwrap_or_default();
    
        if proxy_kv_str.is_empty() {
    
            let req = Fetch::Url(
                Url::parse("https://raw.githubusercontent.com/FoolVPN-ID/Nautica/refs/heads/main/kvProxyList.json")?
            );
    
            let mut res = req.send().await?;
    
            if res.status_code() == 200 {
    
                proxy_kv_str = res.text().await?;
    
                kv.put("proxy_kv", &proxy_kv_str)?
                    .expiration_ttl(60 * 60 * 24)
                    .execute()
                    .await?;
    
            }
    
        }
    
        let proxy_kv: HashMap<String, Vec<String>> =
            serde_json::from_str(&proxy_kv_str)?;
    
        if let Some(list) = proxy_kv.get(&proxyip) {
        
            let mut rand_buf = [0u8; 1];
        
            getrandom(&mut rand_buf)
                .expect("failed generating random number");
        
            let index =
                (rand_buf[0] as usize) % list.len();
        
            proxyip = list[index]
                .replace(":", "-");
        
        }
    
    }
    
    let upgrade =
        req.headers()
            .get("Upgrade")?
            .unwrap_or_default();
    
    return Response::ok(format!(
        "Upgrade={}",
        upgrade
    ));
    
    
    
    if !PROXYIP_PATTERN.is_match(&proxyip) {
    
        return Response::error(
            "Invalid Proxy",
            400,
        );
    
    }
    
    if let Some((addr, port)) = proxyip.split_once('-') {
    
        cx.data.proxy_addr = addr.to_string();
    
        cx.data.proxy_port = port.parse().unwrap_or(443);
    
    }
    
    let WebSocketPair {
    
        server,
    
        client,
    
    } = WebSocketPair::new()?;
    
    server.accept()?;
    
    // jalankan ProxyStream di background
    wasm_bindgen_futures::spawn_local(async move {
    
        let _events = server.events().unwrap();
    
        server.send_with_str("HELLO BLACKOUT").ok();
    
    });
    
    Response::from_websocket(client)

}
