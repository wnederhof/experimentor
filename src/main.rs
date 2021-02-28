use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use experimentor::user_model;
use experimentor::core;
use experimentor::mapper::{convert_user_model_context_to_core, convert_core_toggles_to_user_model};
use std::env;
use std::process::exit;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Syntax: <name> <port>");
        exit(1);
    }

    let filename = args[1].to_string();

    let port = args[2].parse::<u16>().unwrap_or_else(|_| {
        eprintln!("Port is not a valid number.");
        exit(1);
    });

    let file = std::fs::File::open(filename).unwrap_or_else(|_| {
        eprintln!("Unable to open file.");
        exit(1);
    });

    let context: user_model::ContextConfig = serde_yaml::from_reader(file).unwrap_or_else(|err| {
        eprintln!("Unable to parse. Error: {}.", err);
        exit(1);
    });

    println!("Starting server on port {}.", port);
    HttpServer::new(move || {
        App::new().data(context.clone()).route(
            "/feature-toggles/{user_identifier}",
            web::get().to(feature_toggles_handler),
        )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await?;

    Ok(())
}

async fn feature_toggles_handler(
    req: HttpRequest,
    data: web::Data<user_model::ContextConfig>,
) -> impl Responder {
    let user_identifier = req.match_info().get("user_identifier").unwrap();
    web::Json(convert_core_toggles_to_user_model(&core::find_feature_toggles(
        user_identifier,
        &convert_user_model_context_to_core(data.get_ref()),
    )))
}
