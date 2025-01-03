use axum::{
    extract::Query, //extract::Request, handler::HandlerWithoutStateExt, http::StatusCode,
    response::Html,
    routing::get,
    Router,
};
use enigo::{
    Axis::Vertical,
    Button,
    Coordinate::Rel,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use local_ip_address::local_ip;
use open;
//use serde::Deserialize;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // router
    let app = Router::new()
        .nest_service("/", ServeDir::new("assets"))
        .route("/test", get(handler_param_test))
        .route("/e", get(handler_event))
        .route("/author", get(handler_author));

    // listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Axum Listening On {}", listener.local_addr().unwrap());
    //println!("listening on {}", listener. .unwrap());
    let my_local_ip = local_ip().unwrap();
    println!("External Address : http://{:?}:3000", my_local_ip);
    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Deserialize)]
struct Params {
    id: String,
}

//handers

async fn handler_param_test(params: Query<Params>) -> Html<&'static str> {
    println!("{}", params.id);
    Html("rimokon")
}

async fn handler_author() -> Html<&'static str> {
    Html("Sachindra Singh - sachin1618@gmail.com")
}

async fn handler_event(params: Query<Params>) -> Html<&'static str> {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let action: &str = &params.id;
    match action {
        "up" => enigo.move_mouse(0, -60, Rel).unwrap(),
        "dn" => enigo.move_mouse(0, 60, Rel).unwrap(),
        "lt" => enigo.move_mouse(-60, 0, Rel).unwrap(),
        "rt" => enigo.move_mouse(60, 0, Rel).unwrap(),
        "mc" => enigo.button(Button::Left, Click).unwrap(),
        "sup" => enigo.scroll(-10, Vertical).unwrap(),
        "sdn" => enigo.scroll(10, Vertical).unwrap(),
        "o-dp" => open::that("https://disneyplus.com").unwrap(),
        "o-nf" => open::that("https://netflix.com").unwrap(),
        "o-bbc" => open::that("https://bbc.co.uk/iplayer/live/bbcnews").unwrap(),
        "o-pr" => open::that("https://primevideo.com").unwrap(),
        "ks" => enigo.key(Key::Space, Click).unwrap(),
        "kf" => enigo.key(Key::Unicode('f'), Click).unwrap(),
        "kc" => {
            enigo.key(Key::Meta, Press).unwrap();
            enigo.key(Key::Unicode('w'), Click).unwrap();
            enigo.key(Key::Meta, Release).unwrap();
        }
        &_ => todo!(),
    }

    //let action_string = params.id.as_str();
    return Html(Box::leak(action.to_string().into_boxed_str()));
}
