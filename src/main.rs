use std::io;
use walkdir::WalkDir;
use actix_files::{Files, NamedFile};
use chrono::prelude::*;
use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use actix_web::{middleware::Logger};

use std::fs;
use actix_web::{
    body::BoxBody,
    dev::ServiceResponse,
    get,
    http::{header::ContentType, StatusCode},
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    web, App, HttpResponse, HttpServer, Result,
};
use handlebars::Handlebars;
use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Posts {
    name: String,
    posts: Vec<Post>
}
#[derive(Serialize, Deserialize)]
pub struct Post {
    title: String,
    created: String,
    link: String,
    description: String,
    content: String,
    author: String,
}

// Macro documentation can be found in the actix_web_codegen crate
#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    
    // Basic logic here:
    // Every post is an object
    // Each post has all that struct info
    // All the posts make up the blog (TODO rename)
    // In the blog it has basic info and stuff about all the posts
    //let files: Vec<String> = WalkDir::new("./md").into_iter().filter(|dir_entry| dir_entry.as_ref().unwrap().path().is_file()).map(|dir_entry| dir_entry.unwrap().path().to_str().unwrap().to_owned()).collect(); 
    let files: Vec<String> = WalkDir::new("./md").into_iter().filter(|dir_entry| dir_entry.as_ref().unwrap().path().is_file()).map(|dir_entry| dir_entry.unwrap().path().to_str().unwrap().to_owned()).collect(); 
    let mut posts: Vec<Post> = Vec::new();
    for f in &files {
        let attr = fs::metadata(f).unwrap().created().unwrap();
        let datetime = DateTime::<Utc>::from(attr);
        let newdate = datetime.format("%H:%M %d-%m-%Y").to_string();
        // println!("{:?}",newdate.to_string());
        posts.push(Post {
            title: f.to_owned(),
            created: newdate.to_string(),
            link: "path_here_i_think".to_string(),
            description: "brief_summary".to_string(),
            content: "to_be_determined".to_string(),
            author: "caret".to_string(),
        })
    }
    // Put the files array into JSON format for the HTML render
    let all_posts = Posts {
        name: "Blog Posts:".to_string(),
        posts: posts,
    };
    let data = json!(&all_posts);
    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/{user}/{data}")]
async fn user(hb: web::Data<Handlebars<'_>>, path: web::Path<(String, String)>) -> HttpResponse {
    let info = path.into_inner();
    let data = json!({
        "user": info.0,
        "data": info.1
    });
    let body = hb.render("user", &data).unwrap();

    HttpResponse::Ok().body(body)
}
//////////////////////////////////////////////////////

// Custom error handlers, to return HTML responses when an error occurs.
fn error_handlers() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse<BoxBody> {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |e: &str| {
        HttpResponse::build(res.status())
            .content_type(ContentType::plaintext())
            .body(e.to_string())
    };

    let hb = request
        .app_data::<web::Data<Handlebars>>()
        .map(|t| t.get_ref());
    match hb {
        Some(hb) => {
            let data = json!({
                "error": error,
                "status_code": res.status().as_str()
            });
            let body = hb.render("error", &data);

            match body {
                Ok(body) => HttpResponse::build(res.status())
                    .content_type(ContentType::html())
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Handlebars uses a repository for the compiled templates. This object must be
    // shared between the application threads, and is therefore passed to the
    // Application Builder as an atomic reference-counted pointer.
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);
    // Enable logs
    env_logger::init_from_env(env_logger::Env::new().
    default_filter_or("info"));
    // Make server
    HttpServer::new(move || {
        App::new()
            .wrap(error_handlers())
            .wrap(Logger::new("%a '%r' %s %b '%{Referer}i' '%{User-Agent}i' %T").log_target("http_log"))
            .app_data(handlebars_ref.clone())
            .service(Files::new("/static", "static").show_files_listing())
            .service(index)
            .service(user)
    })
    .bind(("0.0.0.0", 38080))?
    .run()
    .await
}