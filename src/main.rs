use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use std::process::exit;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = args[0].to_string();
    let port = args[1].parse::<u16>().unwrap();

    let file = std::fs::File::open(filename).unwrap_or_else(|_| {
        eprintln!("Unable to read toggles.yml.");
        exit(1);
    });

    let context: experimentor::Context = serde_yaml::from_reader(file).unwrap_or_else(|_| {
        eprintln!("Unable to parse toggles.yml.");
        exit(1);
    });

    HttpServer::new(move || {
        App::new()
            .data(context.clone())
            .route("/feature-toggles/{user_identifier}", web::get().to(feature_toggles))
    })
        .bind(("127.0.0.1", port))?
        .run()
        .await?;

    Ok(())
}

async fn feature_toggles(req: HttpRequest, data: web::Data<experimentor::Context>) -> impl Responder {
    let user_identifier = req.match_info().get("user_identifier").unwrap();
    web::Json(experimentor::find_feature_toggles(user_identifier, data.get_ref()))
}
