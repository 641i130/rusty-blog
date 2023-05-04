use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{
    body::BoxBody,
    dev::ServiceResponse,
    get,
    http::{header::ContentType, StatusCode},
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    web, App, HttpResponse, HttpServer, Result,
};
use chrono::prelude::DateTime;
use chrono::Utc;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::io;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;

use std::fs::File;
use std::path::Path;
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Post processor for the markdown files
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

async fn convert(md_file: &str) -> String {
    // This function converts markdown syntax into HTML
    let mut out = String::new(); // HTML output
    let input_filename = Path::new(md_file); //open file
    let file = File::open(&input_filename).expect("[ ERROR ] Failed to open file!");
    // Create a place to store all our tokens
    let reader = BufReader::new(file);
    let mut c_block = false;
    let mut ol_block = false;
    let mut ul_block = false;
    for line in reader.lines() {
        let line_contents = line.unwrap();
        let mut out_line = String::new();
        let line_string = &line_contents.to_string();
        let slice = &line_string.clone();
        match slice {
            s if s.starts_with("#####") && !c_block && !ol_block && !ul_block => {
                out_line.push_str("<h6>");
                out_line.push_str(&s[6..]);
                out_line.push_str("</h6>");
            }
            s if s.starts_with("#####") && !c_block && !ol_block && !ul_block => {
                out_line.push_str("<h5>");
                out_line.push_str(&s[5..]);
                out_line.push_str("</h5>");
            }
            s if s.starts_with("####") && !c_block && !ol_block && !ul_block => {
                out_line.push_str("<h4>");
                out_line.push_str(&s[4..]);
                out_line.push_str("</h4>");
            }
            s if s.starts_with("###") && !c_block && !ol_block && !ul_block => {
                out_line.push_str("<h3>");
                out_line.push_str(&s[3..]);
                out_line.push_str("</h3>");
            }
            s if s.starts_with("##") && !c_block && !ol_block && !ul_block => {
                out_line.push_str("<h2>");
                out_line.push_str(&s[2..]);
                out_line.push_str("</h2>");
            }
            s if s.starts_with("#") && !c_block && !ol_block && !ul_block => {
                out_line.push_str("<h1>");
                out_line.push_str(&s[1..]);
                out_line.push_str("</h1>");
            }
            s if s.starts_with(">") && !c_block && !ol_block && !ul_block => {
                out_line.push_str("<blockquote>");
                out_line.push_str(&s[1..]);
                out_line.push_str("</blockquote>");
            }
            s if s.starts_with("*") || s.starts_with("-") && !c_block && !ol_block && !ul_block => {
                if ul_block {
                    ul_block = false;
                    out_line.push_str("</ul>");
                } else {
                    out_line.push_str("<ul>");
                    ul_block = true;
                }
                out_line.push_str(&s[1..]);
            }
            s if s.starts_with("---") && !c_block && !ol_block && !ul_block => out_line.push_str("<hr>"),

            s if s.starts_with("```") | s.starts_with("`") | s.starts_with("``") => {
                if c_block {
                    c_block = false;
                    out_line.push_str("</pre></code>");
                } else {
                    let lang: String = s.split("```").collect();
                    out_line.push_str(&format!("<pre><code class='language-{}'>", lang.to_string()));
                    c_block = true;
                }
            }
            _ => {
                if c_block {
                    let lines: Vec<&str> = slice.split('\n').collect();
                    for line in lines {
                        let mut escaped_line = String::new();
                        for c in line.chars() {
                            match c {
                                '<' => escaped_line.push_str("&lt;"),
                                '>' => escaped_line.push_str("&gt;"),
                                _ => escaped_line.push(c),
                            }
                        }
                        out_line.push_str(&escaped_line);
                        out_line.push('\n');
                    }
                } else {
                    out_line.push_str("<p>");
                    out_line.push_str(slice);
                    out_line.push_str("</p>");
                }
            }
        }
        out.push_str(&format!("{}", out_line));
    }
    out
}

#[derive(Serialize, Deserialize)]
pub struct Posts {
    name: String,
    posts: Vec<Post>,
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
    let files: Vec<String> = WalkDir::new("./md")
        .into_iter()
        .filter(|dir_entry| dir_entry.as_ref().unwrap().path().is_file())
        .map(|dir_entry| dir_entry.unwrap().path().to_str().unwrap().to_owned())
        .collect();
    let mut posts: Vec<Post> = Vec::new();
    for f in &files {
        let attr = fs::metadata(f).unwrap().created().unwrap();
        let created_date = DateTime::<Utc>::from(attr).format("%H:%M %d-%m-%Y").to_string();
        posts.push(Post {
            title: f.to_owned(),
            created: created_date.to_string(),
            link: "path_here_i_think".to_string(),
            description: "brief_summary".to_string(),
            content: convert(&f).await, // READ FILE IN HERE!!!!
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
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(res.into_parts().0, response.map_into_left_body())))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse<BoxBody> {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |e: &str| HttpResponse::build(res.status()).content_type(ContentType::plaintext()).body(e.to_string());

    let hb = request.app_data::<web::Data<Handlebars>>().map(|t| t.get_ref());
    match hb {
        Some(hb) => {
            let data = json!({
                "error": error,
                "status_code": res.status().as_str()
            });
            let body = hb.render("error", &data);

            match body {
                Ok(body) => HttpResponse::build(res.status()).content_type(ContentType::html()).body(body),
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
    handlebars.register_templates_directory(".html", "./static/templates").unwrap();
    let handlebars_ref = web::Data::new(handlebars);
    // Enable logs
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
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
