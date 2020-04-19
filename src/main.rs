/*
 * Start routine RUST Comment Service
 * (C) 2020 @notessensei, Apache 2.0 license
 */
extern crate chrono;
extern crate config;

mod comments_config;
mod comments_entry;

use actix_web::*;
use chrono::Utc;
use comments_entry::BlogComment;
use comments_entry::BlogSubmission;
use std::collections::HashMap;

const COMMENT_PATH: &str = "/blogcomments/*";

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .status(http::StatusCode::NOT_FOUND)
        .body("Move along, nothing to see here ")
}

async fn post_comment(comment_post: web::Json<BlogComment>, _req: HttpRequest) -> HttpResponse {
    let mut params: HashMap<String, String> = HashMap::new();
    // Client IP
    let ci = &_req.connection_info();
    params.insert("ClientIP".to_string(), ci.remote().unwrap().to_string());
    // All Header parameters
    let headers = _req.headers();
    for (key, value) in headers.iter() {
        params.insert(key.to_string(), value.to_str().unwrap().to_string());
    }

    let submission: BlogSubmission = BlogSubmission {
        created: Utc::now().format("%B %m, %Y %r").to_string(),
        markdown: true,
        parameters: params,
        Commentor: comment_post.Commentor.clone(),
        eMail: comment_post.eMail.clone(),
        webSite: comment_post.webSite.clone(),
        Body: comment_post.Body.clone(),
        captcha: comment_post.captcha.clone(),
        parentId: comment_post.parentId.clone(),
    };

    HttpResponse::Ok().json(submission)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let cfg = comments_config::CommentsConfig::new();
    println!("Running on port {}", cfg.port);
    HttpServer::new(|| {
        App::new().route("/*", web::get().to(index)).service(
            web::resource(COMMENT_PATH).name("new_comment").route(
                web::route()
                    .guard(guard::Post())
                    .guard(guard::Header("Content-Type", "application/json"))
                    .to(post_comment),
            ),
        )
    })
    .bind(("0.0.0.0", cfg.port))?
    .run()
    .await
}
