use worker::*;

#[event(fetch)]
async fn fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    let url = req.url()?;

    match url.path() {
        "/" => {
            let main_url = env.var("MAIN_PAGE_URL")?.to_string();

            let mut resp = Fetch::Url(main_url.parse()?).send().await?;

            Response::from_html(resp.text().await?)
        }

        "/sub" => {
            let sub_url = env.var("SUB_PAGE_URL")?.to_string();

            let mut resp = Fetch::Url(sub_url.parse()?).send().await?;

            Response::from_html(resp.text().await?)
        }

        "/link" => {
            Response::ok("Coming Soon")
        }

        _ => Response::error("404 Not Found", 404),
    }
}
