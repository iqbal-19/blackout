use worker::*;
use crate::config::Config;

pub async fn handle(
    req: Request,
    env: Env,
) -> Result<Response> {

    let cfg = Config::from_env(&env)?;

    let path = req.url()?.path().to_string();

    match path.as_str() {

        "/" => {

            let mut resp = Fetch::Url(
                cfg.main_page_url.parse()?
            )
            .send()
            .await?;

            Response::from_html(resp.text().await?)
        }

        "/sub" => {

            let mut resp = Fetch::Url(
                cfg.sub_page_url.parse()?
            )
            .send()
            .await?;

            Response::from_html(resp.text().await?)
        }

        "/link" => {

            Response::ok("Coming Soon")

        }

        _ => {

            Response::error("404 Not Found",404)

        }

    }

}
