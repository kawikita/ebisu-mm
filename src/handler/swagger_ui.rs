use crate::entity;
use crate::handler;
use actix_web::{HttpResponse, get, web};
use log::debug;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

const API_BASE_PATH: &str = "/api/docs/{_:.*}";

pub fn set_route(cfg: &mut web::ServiceConfig) {
    debug!("Setting up Swagger UI route configurations.");
    cfg.service(SwaggerUi::new(API_BASE_PATH).url("/api/openapi.json", ApiDoc::openapi()));
    cfg.service(openapi_json);
    debug!("Swagger UI route configurations set up successfully.");
}

#[get("/api/openapi.json")]
pub async fn openapi_json() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().json(ApiDoc::openapi()))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::accounts::create_account_handler,
        handler::accounts::get_accounts_list_all_handler,
        handler::accounts::get_accounts_list_by_type_handler,
        handler::accounts::get_account_by_id_handler,
        handler::accounts::update_account_handler,
        handler::accounts::delete_account_handler
    ),
    components(
        schemas(
            entity::accounts::Account,
            entity::accounts::AccountType,
        )
    ),
    tags(
        (name = "accounts", description = "Account management endpoints"),
    ),
    info(
        title = "Ebisu API",
        version = "1.0.0",
        description = "API documentation for the Ebisu application",
        contact(
            name = "Samurai QA Laboratory",
            url = "https://samuraiqalabo.tech/",
            email = "support@samuraiqalabo.tech"
        ),
        license(name = "MIT", url = "https://opensource.org/license/mit/")
    )
)]
pub struct ApiDoc;
