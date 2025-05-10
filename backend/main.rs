use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use applicants::service::{AppState, load_applicants};
use auth::{service::login, models::AuthConfig, middleware::validator};

mod auth;
mod applicants;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let auth_config = AuthConfig::new(
        &std::env::var("JWT_SECRET").expect("JWT_SECRET must be set")
    );

    let initial_data = load_applicants().await;
    let app_state = web::Data::new(AppState {
        applicants: Mutex::new(initial_data),
    });

    HttpServer::new(move || {
        let auth_middleware = HttpAuthentication::bearer(validator);
        
        App::new()
            .app_data(app_state.clone())
            .app_data(web::Data::new(auth_config.clone()))
            .service(login)
            .service(
                web::scope("/api")
                    .wrap(auth_middleware)
                    .service(
                        web::scope("/applicants")
                            .service(applicants::service::get_applicants)
                            .service(applicants::service::update_applicant),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}