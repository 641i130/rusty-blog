use std::io;
use walkdir::WalkDir;
use std::path::Path;
use std::ffi::OsStr;
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

// Macro documentation can be found in the actix_web_codegen crate
#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    //let mut files = Vec::new(); // Init list of files
    // Find all MD files
    //let files: Vec<String> = std::fs::read_dir("./md").unwrap().map(|dir_entry| dir_entry.unwrap().path().to_str().unwrap().to_owned()).collect();
    let files: Vec<String> = WalkDir::new("./md").into_iter().filter(|dir_entry| dir_entry.as_ref().unwrap().path().is_file()).map(|dir_entry| dir_entry.unwrap().path().to_str().unwrap().to_owned()).collect(); 
    //for file in WalkDir::new("./md").into_iter().filter_map(|file| file.ok()) {
        //if file.metadata().unwrap().is_file() && file.path().extension().unwrap() == "md" {
            //println!("{:?}",file.to_string_lossy().into_owned());
            /*match file.file_name().to_str() {
                Some(name) => files.push(name.to_owned()),
                _ => println!("odd file error"),
            }*/
        //}
   // }
    println!("{:?}",&files);
    // Put the files array into JSON format for the HTML render
    let data = json!({
        "name": "Blog",
        // insert all MD posts
    });
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

    HttpServer::new(move || {
        App::new()
            .wrap(error_handlers())
            .app_data(handlebars_ref.clone())
            .service(index)
            .service(user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}