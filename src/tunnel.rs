use worker::*;
use crate::config::Config;

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

    Response::ok(format!("Tunnel Ready : {}", proxyip))

}
