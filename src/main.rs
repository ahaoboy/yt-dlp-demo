use salvo::prelude::*;
use tool::version;
mod tool;

#[handler]
async fn get_version(res: &mut Response) {
    res.render(Json(version().await));
}
use rust_embed::RustEmbed;
use salvo::serve_static::static_embed;

#[derive(RustEmbed)]
#[folder = "dist"]
struct Assets;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;

    let router = Router::new()
        .push(Router::with_path("get_version").get(get_version))
        .push(
            Router::with_path("{*path}")
                .push(Router::with_path("get_version").get(get_version))
                .get(static_embed::<Assets>().fallback("index.html")),
        );

    println!("{:?}", router);

    Server::new(acceptor).serve(router).await;
}
