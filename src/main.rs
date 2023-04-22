use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    error,
    http::StatusCode,
    middleware, web, App, HttpMessage as _, HttpRequest, HttpServer, Responder,
    HttpResponse,
};
use actix_web::error::HttpError;
use actix_files::Files;
use actix_session::config;
// until request processing and configuration loading is used
// use dotenv::dotenv;
// use crate::config;
const ONE_MINUTE: Duration = Duration::minutes(1);

async fn index(identity: Option<Identity>) -> actix_web::Result<impl Responder> {
    let id = match identity.map(|id| id.id()) {
        None => "anonymous".to_owned(),
        Some(Ok(id)) => id,
        Some(Err(err)) => return Err(error::ErrorInternalServerError(err)),
    };

    Ok(format!("Hello {id}"))
}

async fn login(req: HttpRequest) -> impl Responder {
    Identity::login(&req.extensions(), "user1".to_owned()).unwrap();

    web::Redirect::to("/").using_status_code(StatusCode::FOUND)
}

async fn logout(id: Identity) -> impl Responder {
    id.logout();

    web::Redirect::to("/").using_status_code(StatusCode::FOUND)
}
async fn something() -> HttpResponse {
    HttpResponse::Ok().finish()
}
// async fn submit(req: HttpRequest, ) -> actix_web::Result<HttpResponse, HttpError> {
//     let params = req.match_info();
//     let id = req.extensions().get::<Identity>().unwrap();
//     let id = id.id();
//     let id = id?.as_str();
//
// }
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenv().ok();  - for load config from .env
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let secret_key = Key::generate();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/logout").route(web::post().to(logout)))
            .service(web::resource("/").route(web::get().to(index)))
            .service(Files::new("/static", "static/params/").show_files_listing())
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("auth-example".to_owned())
                    .cookie_secure(false) //until we think about security
                    .session_lifecycle(PersistentSession::default().session_ttl(ONE_MINUTE))
                    .build(),
            )
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
