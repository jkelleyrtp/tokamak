use cookie::Cookie;
use http::StatusCode;
use tokamak::{App, Request, Response, ToResponse};

#[tokio::main]
async fn main() {
    let mut app = App::default();

    app.at("/")
        .get(|req: Request| format!("hello cookies: {:?}", req.cookie("hello")?).to_response());

    app.at("/set").get(|_| {
        Response::new(StatusCode::OK)
            .with_cookie(Cookie::new("hello", "world"))
            .build()
    });

    app.at("/remove").get(|_| {
        Response::new(StatusCode::OK)
            .with_remove_cookie(Cookie::named("hello"))
            .build()
    });

    app.listen("127.0.0.1:8080").await.unwrap();
}