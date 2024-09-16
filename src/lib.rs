use reqwest::Client;
use serde_json::{json, Value};
use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    router
        .get_async("/", |_, _| async move {
            Response::ok("How to use: http://xxx.xxx/SongID\nExample: https://netease-url.genshinminecraft-d20.workers.dev/406716121")
        })
        .get_async("/:id", get_163).run(req, env).await
}


pub async fn get_163(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let client = Client::new();

    let id = match ctx.param("id") {
        Some(id) => match id.parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Response::error("Invalid ID", 400),
        },
        None => return Response::error("Invalid ID", 400),
    };

    let url = format!("https://oiapi.net/API/Music_163?id={}&n=1", id);

    let resp = match client.get(&url).send().await {
        Ok(resp) => resp,
        Err(_) => return Response::error("Failed to fetch data", 500),
    };

    let json: Value = match resp.json().await {
        Ok(json) => json,
        Err(_) => return Response::error("Failed to parse JSON", 500),
    };

    let data = match json["data"].as_array() {
        Some(data) => data,
        None => return Response::error("No \"data\" found in JSON", 500),
    };

    let song = data.get(0).unwrap();

    let url = song["url"].to_string();

    let url = &url[1..url.len()-1];

    Response::ok(url)
}
