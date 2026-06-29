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
    ).unwrap();

    let config = Config {
        uuid,
        host: String::new(),
        proxy_addr: String::new(),
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
        .get_async("/link", |req, ctx| async move {
            router::handle(req, ctx.env).await
        })
        .on_async("/:proxyip", tunnel)
        .on_async("/vmess", tunnel)
        .on_async("/vless", tunnel)
        .on_async("/trojan", tunnel)
        .on_async("/shadowsocks", tunnel)
        .run(req, env)
        .await
}

async fn tunnel(
    _req: Request,
    _ctx: RouteContext<Config>,
) -> Result<Response> {

    Response::ok("Tunnel OK")

}
