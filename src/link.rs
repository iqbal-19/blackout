use worker::*;
use crate::config::Config;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde_json::json;

pub async fn handle(
    _: Request,
    ctx: RouteContext<Config>,
) -> Result<Response> {

    let host = ctx.data.host.to_string();
    let uuid = ctx.data.uuid.to_string();

    let vmess_link = {
        let config = json!({
            "ps": "blackout vmess",
            "v": "2",
            "add": host,
            "port": "80",
            "id": uuid,
            "aid": "0",
            "scy": "zero",
            "net": "ws",
            "type": "none",
            "host": host,
            "path": "/ID",
            "tls": "",
            "sni": "",
            "alpn": ""
        });

        format!("vmess://{}", URL_SAFE.encode(config.to_string()))
    };

    let vless_link = format!(
        "vless://{uuid}@{host}:443?encryption=none&type=ws&host={host}&path=%2FID&security=tls&sni={host}#blackout vless"
    );

    let trojan_link = format!(
        "trojan://{uuid}@{host}:443?encryption=none&type=ws&host={host}&path=%2FID&security=tls&sni={host}#blackout trojan"
    );

    let ss_link = format!(
        "ss://{}@{host}:443?plugin=v2ray-plugin%3Btls%3Bmux%3D0%3Bmode%3Dwebsocket%3Bpath%3D%2FID%3Bhost%3D{host}#blackout ss",
        URL_SAFE.encode(format!("none:{uuid}"))
    );

    Response::ok(format!(
        "{}\n{}\n{}\n{}",
        vmess_link,
        vless_link,
        trojan_link,
        ss_link
    ))
}
