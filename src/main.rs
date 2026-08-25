use std::{str::FromStr};

use chrono::{DateTime, TimeZone, Utc};
use rocket::{serde::json::Json};
use rocket::request::Request;
use rocket::http::Status;
use rocket_authorization::{AuthError, Authorization, Credential};
use serde::{Deserialize, Serialize};
use tracing::Level;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use jwt_service_impl::jwt_service_impl::JWTTokenResponse;
use uuid::Uuid;

use crate::api_client_service_impl::api_client_service_impl::{ApiClientCredential};

mod jwt_service_impl;
mod api_client_service_impl;
mod geospatial_computations;
mod categories_service_impl;
mod company_service_impl;

#[macro_use] extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct IndexPageResponse {
    message: String,
    version: String,
    documentation_url: String
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct TokenResponse {
    pub status: u16,
    pub message: String,
    pub errors: Vec<ResponseError>,
    pub data: JWTTokenResponse
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct ResponseError {
    pub error_code: String,
    pub error_message: String
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct TokenRequest {
    client_id: String,
    client_secret: String,
    grant_type: String
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct CatchResponse {
    pub status: u16,
    pub message: String,
    pub errors: Vec<ResponseError>,
    pub data: Vec<String>
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct CreateApiCredentialRequest {
    pub client_id: String,
    pub is_active: bool
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct CreateApiClientCredentialResponse {
    pub status: u16,
    pub message: String,
    pub errors: Vec<ResponseError>,
    pub data: Vec<ApiClientCredential>
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct ListApiClientCredentials {
    pub status: u16,
    pub message: String,
    pub limit: i32,
    pub offset: i32,
    pub total_count: i32,
    pub errors: Vec<ResponseError>,
    pub data: Vec<ApiClientCredential>
}

#[derive(FromForm)]
struct FilterOptions {
    search: String,
    offset: Option<usize>,
    limit: Option<usize>,
}

#[derive(Debug)]
pub struct CustomAuthentication {
    pub subject: String
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiClientPermissionResponse {
    pub status: u16,
    pub message: String,
    pub limit: i32,
    pub offset: i32,
    errors: Vec<ResponseError>,
    pub data: Vec<ApiClientPermission>
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiClientPermission {
    pub client_id: String,
    pub permissions: Vec<String>
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateApiClientPermission {
    pub client_id: String,
    pub permissions: Vec<ApiClientPermission>
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateApiClientPermission {

}


#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct ProductCategoriesResponse {
    status: u16,
    message: String,
    limit: i32,
    offset: i32,
    total_count: i32,
    errors: Vec<ResponseError>,
    data: Vec<ProductCategory>
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct ProductCategory {
    id: String,
    company_id: String, 
    name: String,
    description: String,
    parent_category_id: String,
    created_at: String,
    modified_at: String,
    is_active: bool  
}

#[rocket::async_trait]
impl Authorization for CustomAuthentication {
    const KIND: &'static str = "Bearer";

    async fn parse(_: &str, credential: &str, request: &Request) -> Result<Self, AuthError> { 
        
        if credential.is_empty() {
            return Err(AuthError::HeaderMissing);
        }

        let uri = request.uri();

        let decode_token: Result<jwt_service_impl::jwt_service_impl::Claims, jwt_service_impl::jwt_service_impl::ClaimsError> = jwt_service_impl::jwt_service_impl::decode_token(&credential.to_string()).await;

        match decode_token {
            Ok(claims) => {
                let expires_in = claims.exp;

                let expires: DateTime<Utc> = Utc.timestamp_opt(expires_in as i64, 0).unwrap();

                let current_date_time: DateTime<Utc> = Utc::now();

                let issued_on = claims.iss;

                //check if token is not expired
                if current_date_time > expires {
                    return Err(AuthError::Forbidden);
                } else {
                    //check if api client is enabled on the platform
                    let is_allowed: Result<bool, api_client_service_impl::api_client_service_impl::IsClientValidError> = api_client_service_impl::api_client_service_impl::is_allowed(claims.sub.as_str()).await;

                    match is_allowed {
                        Ok(resp) => {
                            if resp == true {
                                tracing::info!("audience presented in token : {} and subject : {} expires: {} issued_on: {}", claims.aud, claims.sub, expires_in, issued_on);
                                return Ok(CustomAuthentication { subject: claims.sub })
                            } else {
                                return Err(AuthError::Forbidden);
                            }
                        },
                        Err(e) => {
                            tracing::warn!("error occured while decoding token, message : {}", e.error_message);
                            return Err(AuthError::Unauthorized);
                        }
                    }
                }
            },
            Err(err) => {
                tracing::warn!("error occured while decoding token, uri: {} message : {}", uri, err.error_message);
                return Err(AuthError::Unauthorized);
            }
        };
    }
}

#[get("/", format="json")]
async fn index() -> Json<IndexPageResponse> {
    tracing::info!("initiating serving index page");

    let response = IndexPageResponse {
        message: String::from("Welcome to Project Eneo Restful API Service"),
        version: String::from("0.1.1a"),
        documentation_url: String::from("")
    };

    tracing::info!("completing serving index page");

    return Json(response)
}

#[post("/v1/authenticate", format="json", data="<token_request>")]
async fn get_authentication_token(token_request: Json<TokenRequest>) -> (Status, Json<TokenResponse>) {
    
    let _client_id = &token_request.client_id;
    let _client_secret = &token_request.client_secret;
    let _grant_type = &token_request.grant_type;

    let mut token_data: JWTTokenResponse = JWTTokenResponse { 
        access_token: String::from(""), 
        token_type: String::from(""), 
        expires_in: 00000, 
        refresh_token: String::from(""), 
        scope: String::from(""), 
        state: String::from("") };
    
    let mut status_code: u16 = 200;

    let mut errors: Vec<ResponseError> = Vec::with_capacity(1);

    let token_response: Result<JWTTokenResponse, jwt_service_impl::jwt_service_impl::JWTErrorResponse> = jwt_service_impl::jwt_service_impl::token(_client_id, _client_secret, _grant_type).await;

    match token_response {
        Ok(resp) => {
            token_data = resp
        },
        Err(err) => {
            status_code = 400;

            let response_error = ResponseError {
                error_code: err.error_code,
                error_message: err.error_message
            };
            
            errors.push(response_error);
        }
    }

    let response = TokenResponse {
        status: status_code,
        errors: errors,
        message: Status::from_code(status_code).unwrap().reason().unwrap().to_string(),
        data: token_data
    };

    tracing::info!("completing processing authentication request. status code: {}", status_code);

    return (Status::from_code(status_code).unwrap(), Json(response));
}

#[post("/v1/client/credentials", format="json", data="<create_api_credential_request>")]
async fn create_api_credential(create_api_credential_request: Json<CreateApiCredentialRequest>, auth: Credential<CustomAuthentication>) -> (Status, Json<CreateApiClientCredentialResponse>) {
    let mut status_code: u16 = 200;

    let subject = &auth.subject;

    tracing::info!("initiating create api client credential by: {}", subject);

    let generated_client_secret: api_client_service_impl::api_client_service_impl::EncryptedClientSecret = api_client_service_impl::api_client_service_impl::generate_and_encrypt_client_secret().await.unwrap();
    
    let mut errors: Vec<ResponseError> = Vec::with_capacity(1);

    let mut data: Vec<ApiClientCredential> = Vec::with_capacity(1);

    let create_api_credential_result = api_client_service_impl::api_client_service_impl::create_credential(&create_api_credential_request.client_id, &generated_client_secret.encrypted_secret, &create_api_credential_request.is_active).await;

    match create_api_credential_result {
        Ok(client) => {
            data.push(client);
        },
        Err(error) => {
            status_code = error.error_code.parse::<u16>().expect("Not a valid u16");
            errors.push(ResponseError { error_code: error.error_code, error_message: error.error_message });
        }
    }

    let response = CreateApiClientCredentialResponse {
        status: status_code,
        message: String::from(Status::from_code(status_code).unwrap().reason().unwrap()),
        errors: errors,
        data: data
    };

    return (Status::from_code(status_code).unwrap(), Json(response));
}

#[delete("/v1/client/credentials/<id>", format="json")]
async fn delete_api_client_credential(id: &str, auth: Credential<CustomAuthentication>) -> (Status, Json<CreateApiClientCredentialResponse>) {
    let mut http_status: u16 = 200;

    let subject = &auth.subject;

    tracing::info!("initiating delete api client credential by: {}", subject);

    let id_as_uuid = Uuid::from_str(id);

    let mut errors: Vec<ResponseError> = Vec::with_capacity(1);
    let mut data: Vec<ApiClientCredential> = Vec::with_capacity(1);

    match id_as_uuid {
        Ok(client_id) => {
            
            let delete_response = api_client_service_impl::api_client_service_impl::delete_credential(&client_id).await;

            match delete_response {
                Ok(response) => {
                    data.push(response);
                },
                Err(err) => {
                    http_status = 400;

                    errors.push(ResponseError { 
                        error_code: err.error_code, 
                        error_message: err.error_message 
                    });
                }
            }
        },
        Err(e) => {
            http_status = 400;
            errors.push(ResponseError { error_code: http_status.to_string(), error_message: e.to_string() });
        }
    }
    
    let response = CreateApiClientCredentialResponse {
        status: http_status,
        message: String::from(Status::from_code(http_status).unwrap().reason().unwrap()),
        errors: errors,
        data: Vec::new()
    };

    return (Status::from_code(http_status).unwrap(), Json(response));
}

#[get("/v1/client/credentials?<filters..>", format="json")]
async fn get_api_client_credentials(filters: FilterOptions, auth: Credential<CustomAuthentication>) -> (Status, Json<ListApiClientCredentials>) {
    let mut status = 200;

    let subject = &auth.subject;

    tracing::info!("initiating get api client credentials by: {}", subject);

    let offset: i32 = filters.offset.unwrap() as i32;
    let limit: i32 = filters.limit.unwrap() as i32;
    let search: &str = filters.search.as_str();

    let list_of_api_client_credentials = api_client_service_impl::api_client_service_impl::get_api_client_credentials(search, offset, limit).await;
    
    let mut data: Vec<ApiClientCredential> = Vec::with_capacity(limit as usize);

    let mut errors: Vec<ResponseError> = Vec::with_capacity(1);

    match list_of_api_client_credentials {
        Ok(api_client_credentials) => {
            data.extend(api_client_credentials);
        },
        Err(err) => {
            status = 400;

            errors.push(ResponseError { 
                error_code: err.error_code, 
                error_message: err.error_message 
            });
        }
    }

    let response: ListApiClientCredentials = ListApiClientCredentials {
        status: status,
        errors: errors,
        message: Status::from_code(status).unwrap().reason().unwrap().to_string(),
        limit: filters.limit.unwrap() as i32,
        offset: filters.offset.unwrap() as i32,
        total_count: 0,
        data: data
    };

    return (Status::from_code(status).unwrap(), Json(response));
}

//create permissions to an api client by id
#[post("/v1/client/credentials/<id>/permissions", format="json", data="<create_api_client_permission>")]
async fn create_api_client_permission(create_api_client_permission: Json<CreateApiClientPermission>, id: &str, auth: Credential<CustomAuthentication>) -> (Status, Json<ApiClientPermissionResponse>) {
    let mut status = 200;

    let subject = &auth.subject;

    tracing::info!("initiating create api permission by: {} client id: {}", subject, id);

    let mut errors: Vec<ResponseError> = Vec::with_capacity(1);

    let mut data: Vec<ApiClientPermission> = Vec::with_capacity(1);

    

    let response: ApiClientPermissionResponse = ApiClientPermissionResponse {
        status: status,
        errors: errors,
        limit: 0,
        offset: 0,
        message: Status::from_code(status).unwrap().reason().unwrap().to_string(),
        data: data
    };


    return (Status::from_code(status).unwrap(), Json(response));
}

//update permission to an apu client by id
#[put("/v1/client/credentials/<id>/permissions/<permission_id>", format="json", data="<update_api_client_permission>")]
async fn update_api_client_permission(update_api_client_permission: Json<UpdateApiClientPermission>, id: &str, permission_id: &str, auth: Credential<CustomAuthentication>) -> (Status, Json<ApiClientPermissionResponse>) {
    let mut status = 200;

    let subject = &auth.subject;

    tracing::info!("initiating update api permission by: {} client id: {}", subject, id);

    let mut errors: Vec<ResponseError> = Vec::with_capacity(1);

    let mut data: Vec<ApiClientPermission> = Vec::with_capacity(1);

    let response: ApiClientPermissionResponse = ApiClientPermissionResponse {
        status: status,
        errors: errors,
        limit: 0,
        offset: 0,
        message: Status::from_code(status).unwrap().reason().unwrap().to_string(),
        data: data
    };


    return (Status::from_code(status).unwrap(), Json(response));
}

//list all permissions assigned to an api client by id
#[get("/v1/client/credentials/<id>/permissions", format="json")]
async fn get_api_client_permission(id: &str, auth: Credential<CustomAuthentication>) -> (Status, Json<ApiClientPermissionResponse>) {
    let mut status = 200;

    let subject = &auth.subject;

    tracing::info!("initiating get api permissions by: {} client id: {}", subject, id);

    let mut errors: Vec<ResponseError> = Vec::with_capacity(1);

    let mut data: Vec<ApiClientPermission> = Vec::with_capacity(1);

    let response: ApiClientPermissionResponse = ApiClientPermissionResponse {
        status: status,
        errors: errors,
        limit: 0,
        offset: 0,
        message: Status::from_code(status).unwrap().reason().unwrap().to_string(),
        data: data
    };


    return (Status::from_code(status).unwrap(), Json(response));
}

//delete a permission assigned to an api client by id
#[delete("/v1/client/credentials/<id>/permissions/<permission_id>", format="json")]
async fn delete_api_client_permission(id: &str, permission_id: &str, auth: Credential<CustomAuthentication>) -> (Status, Json<ApiClientPermissionResponse>) {
    let mut status = 200;

    let subject = &auth.subject;

    tracing::info!("initiating delete api permission by: {} client id: {} permission id: {}", subject, id, permission_id);

    let mut errors: Vec<ResponseError> = Vec::with_capacity(1);

    let mut data: Vec<ApiClientPermission> = Vec::with_capacity(1);

    let response: ApiClientPermissionResponse = ApiClientPermissionResponse {
        status: status,
        errors: errors,
        limit: 0,
        offset: 0,
        message: Status::from_code(status).unwrap().reason().unwrap().to_string(),
        data: data
    };


    return (Status::from_code(status).unwrap(), Json(response));
}

//get permissiion by id assigned to an api client by id
#[get("/v1/client/credentials/<id>/permissions/<permission_id>", format="json")]
async fn get_api_client_permission_by_id(id: &str, permission_id: &str, auth: Credential<CustomAuthentication>) -> (Status, Json<ApiClientPermissionResponse>) {
    let mut status = 200;

    let subject = &auth.subject;

    tracing::info!("initiating get api credential by: {} client id: {} and id: {}", subject, id, permission_id);

    let mut errors: Vec<ResponseError> = Vec::with_capacity(1);

    let mut data: Vec<ApiClientPermission> = Vec::with_capacity(1);

    let response: ApiClientPermissionResponse = ApiClientPermissionResponse {
        status: status,
        errors: errors,
        limit: 0,
        offset: 0,
        message: Status::from_code(status).unwrap().reason().unwrap().to_string(),
        data: data
    };


    return (Status::from_code(status).unwrap(), Json(response));
}


//get product categories by company_id
#[get("/v1/company/<company_id>/product/categories", format="json")]
async fn get_product_categories_by_company_id(company_id: &str, auth: Credential<CustomAuthentication>) -> (Status, Json<ProductCategoriesResponse>) {
    let mut status = 200;

    let subject = &auth.subject;

    tracing::info!("initiating get product categories by company _id: {}", company_id);

    let mut errors: Vec<ResponseError> = Vec::with_capacity(1);

    let company_uuid = Uuid::from_str(company_id);

    let mut data: Vec<ProductCategory> = Vec::new();

    match company_uuid {
        Ok(_id) => {
            let is_active: bool = true;

            let categories_result = categories_service_impl::categories_service_impl::get_categories_by_company_id(&_id, &is_active).await;

            match categories_result {
                Ok(categories) => {
                    for category in categories {
                        data.push(ProductCategory {
                            company_id: category.company_id.to_string(),
                            created_at: category.created_at.to_string(),
                            id: category.id.to_string(),
                            name: category.name,
                            description: category.description,
                            parent_category_id: "".to_string(),
                            modified_at: category.modified_at.to_string(),
                            is_active: category.is_active
                        })
                    }
                },
                Err(e) => {
                    errors.push(ResponseError { error_code: e.error_code.to_string(), error_message: e.error_message });
                }
            }
        },
        Err(e) => {
            errors.push(ResponseError{ error_code: 400.to_string(), error_message: e.to_string()});
        }
    }

    let response: ProductCategoriesResponse = ProductCategoriesResponse {
        status: status,
        errors: errors,
        limit: 0,
        offset: 0,
        total_count: 0,
        message: Status::from_code(status).unwrap().reason().unwrap().to_string(),
        data: data
    };


    return (Status::from_code(status).unwrap(), Json(response));
}
#[catch(404)]
fn not_found(request: &Request) -> Json<CatchResponse> {

    let error = ResponseError {
        error_code: 400.to_string(),
        error_message: format!("resource not found. uri: {}", request.uri())
    };

    let errors = vec![error];
    let message = String::from_str(Status::from_code(404).unwrap().reason().unwrap());

    return Json(CatchResponse { status: 404, message: message.unwrap(), errors: errors, data: Vec::new() })
}

#[catch(500)]
fn internal_server_error(request: &Request) -> Json<CatchResponse> {
    tracing::warn!("request internal server error: {}", request.uri());

    let error = ResponseError {
        error_code: 500.to_string(),
        error_message: format!("internal server error.")
    };

    let errors = vec![error];
    let message = String::from_str(Status::from_code(500).unwrap().reason().unwrap());

    return Json(CatchResponse { status: 500, message: message.unwrap(), errors: errors, data: Vec::new() })
}

#[catch(401)]
fn unauthorized(request: &Request) -> Json<CatchResponse> {
    tracing::warn!("unauthorized error: {}", request.uri());

    let error = ResponseError {
        error_code: 401.to_string(),
        error_message: format!("unauthorized")
    };

    let errors = vec![error];
    let message = String::from_str(Status::from_code(401).unwrap().reason().unwrap());

    return Json(CatchResponse { status: 401, message: message.unwrap(), errors: errors, data: Vec::new() })
}

#[catch(400)]
fn bad_request(request: &Request) -> Json<CatchResponse> {

    let error = ResponseError {
        error_code: 404.to_string(),
        error_message: format!("bad request not found. uri: {}", request.uri())
    };

    let errors = vec![error];
    let message = String::from_str(Status::from_code(400).unwrap().reason().unwrap());

    return Json(CatchResponse { status: 400, message: message.unwrap(), errors: errors, data: Vec::new() })
}


#[catch(422)]
fn unprocessable_entity(request: &Request) -> Json<CatchResponse> {

    let error = ResponseError {
        error_code: 422.to_string(),
        error_message: format!("Unprocessable Entity. uri: {}", request.uri())
    };

    let errors = vec![error];
    let message = String::from_str(Status::from_code(422).unwrap().reason().unwrap());

    return Json(CatchResponse { status: 400, message: message.unwrap(), errors: errors, data: Vec::new() })
}

#[launch]
fn rocket() -> _ {

    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        "./logs",
        "app.log"
    );

    // tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).with_ansi(false).with_span_events(FmtSpan::CLOSE).init();
    tracing_subscriber::fmt()
    .json()
    .with_ansi(false)
    // .with_env_filter(filter)
    .with_writer(file_appender)
    .flatten_event(true)
    .with_max_level(Level::INFO)
    .with_current_span(true)
    .with_span_list(true)
    .init();

    rocket::build()
    .register("/", catchers![not_found, internal_server_error, unauthorized, bad_request, unprocessable_entity])
    .mount("/", routes![
        index, 
        get_authentication_token, 
        create_api_credential, 
        delete_api_client_credential, 
        get_api_client_credentials,
        get_product_categories_by_company_id
    ])
}
