use actix_files as fs;
use actix_web::{
    cookie, http, middleware, web, App, HttpMessage, HttpResponse, HttpServer, Result,
};
use askama::Template;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "index.html")]
struct Index<'a> {
    theme: &'a str,
    next_theme: &'a str,
}

#[derive(Deserialize)]
struct FormData {
    theme: String,
}

#[derive(Deserialize)]
struct Query {
    theme: Option<String>,
}

fn get_next_theme(theme: &str) -> &str {
    if theme == "dark" {
        "light"
    } else {
        "dark"
    }
}

// fn get_requested_with<'a>(req: &'a web::HttpRequest) -> Option<&'a str> {
//     req.headers().get("x-requested-with")?.to_str().ok()
// }

async fn index(req: web::HttpRequest, query: web::Query<Query>) -> Result<HttpResponse> {
    let theme = match &query.theme {
        Some(theme) => theme.clone(),
        None => match req.cookie("theme") {
            Some(theme) => theme.value().to_string(),
            None => String::from("dark"),
        },
    };

    let cookie = cookie::Cookie::build("theme", theme.clone())
        .secure(true)
        .http_only(true)
        .finish();

    let markup = Index {
        theme: theme.as_str(),
        next_theme: get_next_theme(&theme),
    }
    .render()
    .unwrap();

    let response = HttpResponse::build(http::StatusCode::from_u16(200).unwrap())
        .cookie(cookie)
        .content_type("text/html")
        .body(markup);

    Ok(response)
}

async fn update_theme(form: web::Form<FormData>) -> Result<HttpResponse> {
    let theme = form.theme.clone();

    let cookie = cookie::Cookie::build("theme", theme.clone())
        .secure(true)
        .http_only(true)
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/")
                    .route(web::get().to(index))
                    .route(web::post().to(update_theme)),
            )
            .service(fs::Files::new("/public", "public"))
    })
    .bind("127.0.0.1:4020")?
    .run()
    .await
}
