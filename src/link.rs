use worker::*;
use crate::config::Config;

pub async fn handle(
    _req: Request,
    ctx: RouteContext<Config>,
) -> Result<Response> {

    Response::ok(format!(
        "Host : {}\nUUID : {}",
        ctx.data.host,
        ctx.data.uuid
    ))

}
