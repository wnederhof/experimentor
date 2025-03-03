use actix_web::http::StatusCode;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use experimentor::core;
use experimentor::mapper::{map_contexts_config_to_contexts, map_toggles_to_toggles_response};
use experimentor::user_model;
use serde::Deserialize;
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

    let context: user_model::ContextsConfig = serde_yaml::from_reader(file).unwrap_or_else(|err| {
        eprintln!("Unable to parse. Error: {}.", err);
        exit(1);
    });

    println!("Starting server on port {}.", port);
    HttpServer::new(move || {
        App::new()
            .data(context.clone())
            .route(
                "/contexts/{context_name}/feature-toggles/{user_identifier}",
                web::get().to(feature_toggles_handler),
            )
            .route("/health", web::get().to(health_handler))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await?;

    Ok(())
}

#[derive(Debug, Deserialize, Clone)]
struct FeatureTogglesParams {
    hash: Option<String>,
}

async fn feature_toggles_handler(
    query_params: web::Query<FeatureTogglesParams>,
    req: HttpRequest,
    data: web::Data<user_model::ContextsConfig>,
) -> impl Responder {
    let context_name = req.match_info().get("context_name").unwrap();
    let user_identifier = req.match_info().get("user_identifier").unwrap();
    web::Json(map_toggles_to_toggles_response(
        query_params.hash.as_deref(),
        &core::find_feature_toggles(
            context_name,
            user_identifier,
            &map_contexts_config_to_contexts(data.get_ref()),
        ),
    ))
}

async fn health_handler() -> impl Responder {
    (String::from(""), StatusCode::OK)
}
