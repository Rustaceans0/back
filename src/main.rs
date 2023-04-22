// mod responser;
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{ web,
    cookie::{time::Duration, Key},
    error,
    http::StatusCode,
    middleware, web::Header, App, HttpMessage as _, HttpRequest, HttpServer, Responder,
    HttpResponse,
};
use actix_web::error::HttpError;
use actix_files::Files;
use actix_session::config;
use serde::{Deserialize, Serialize};
use actix_web::web::Redirect;
use actix_web::http::header;
// until request processing and configuration loading is used
// use dotenv::dotenv;
// use crate::config;
const ONE_MINUTE: Duration = Duration::minutes(1);
// fn my_handler() -> HttpResponse {
//
//         let res = HttpResponse::Ok().cookie().finish();
//     }

async fn index(identity: Option<Identity>) -> actix_web::Result<impl Responder> {
    let id = match identity.map(|id| id.id()) {
        None => "anonymous".to_owned(),
        Some(Ok(id)) => id,
        Some(Err(err)) => return Err(error::ErrorInternalServerError(err)),
    };

    Ok(format!("Hello {id}"))
}

async fn login(req: HttpRequest) -> impl Responder {
    Identity::login(&req.extensions(), "Alex".to_owned()).unwrap();


     Redirect::to("/").using_status_code(StatusCode::FOUND)
}
async fn register(req: HttpRequest) -> impl Responder {
    Identity::register(&req.extensions().to_owned()).unwrap();
    Redirect::to("/").using_status_code(StatusCode::FOUND)
}
async fn loggin(id: Identity) -> impl Responder {
    id.login(&id.logout(), "nobody".to_owned()).unwrap();
}
async fn logout(id: Identity) -> impl Responder {
    id.logout();

    Redirect::to("/").using_status_code(StatusCode::FOUND)
}
async fn something() -> HttpResponse {
    HttpResponse::Ok().finish()
}
// async fn submit(req: HttpRequest, ) -> HttpResponse {
//      let params = req.match_info();
//      let id = req.extensions().get::<Identity>().unwrap();
//      let id = id.id();
//      let id = id?.as_str();
//
//  }
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenv().ok();  - for load config from .env
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let secret_key = Key::generate();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/loggin").route(web::post().to(loggin)))
            // .service (Files::new("/params", "~/CLionProjects/back/").())
            .service(web::resource("/logout").route(web::post().to(logout)))
            .service(web::resource("/").route(web::get().to(index)))
            .service(Files::new("/index", "/").index_file("index.html"))
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
    .bind(("10.2.0.71", 8080))?
    .run()
    .await
}
