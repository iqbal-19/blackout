mod config;
mod router;

use worker::*;

#[event(fetch)]
async fn fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {

    router::handle(req, env).await

}
