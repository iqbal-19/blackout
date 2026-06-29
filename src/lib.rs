mod link;
mod links;
mod common;
mod config;
mod proxy;
mod router;
mod tunnel;

use config::Config;
use uuid::Uuid;
use worker::*;

#[event(fetch)]
async fn fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {

    let uuid = Uuid::parse_str(
        &env.var("UUID")?.to_string()
    ).unwrap_or_default();

    let host = match req.url()?.host() {
        Some(h) => h.to_string(),
        None => String::new(),
    };
    
    let config = Config {
        uuid,
        host: host.clone(),
        proxy_addr: host,
        proxy_port: 443,
        main_page_url: env.var("MAIN_PAGE_URL")?.to_string(),
        sub_page_url: env.var("SUB_PAGE_URL")?.to_string(),
    };
    

    Router::with_data(config)
        .get_async("/", |req, ctx| async move {
            router::handle(req, ctx.env).await
        })
        .get_async("/sub", |req, ctx| async move {
            router::handle(req, ctx.env).await
        })
        .on_async("/link", link::handle)
        .on_async("/:proxyip", tunnel::handle)
        .on_async("/vmess", tunnel::handle)
        .on_async("/vless", tunnel::handle)
        .on_async("/trojan", tunnel::handle)
        .on_async("/shadowsocks", tunnel::handle)
        .run(req, env)
        .await
}

async fn tunnel(
    _req: Request,
    _ctx: RouteContext<Config>,
) -> Result<Response> {

    Response::ok("Tunnel OK")

}
