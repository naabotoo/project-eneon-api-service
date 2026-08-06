
pub mod jwt_service_impl {
    use chrono::{DateTime, Local, TimeDelta, Utc};
    use core::{str};
    use dotenvy::dotenv;
    use jsonwebtoken::{EncodingKey, Header, encode};
    use serde::{Deserialize, Serialize};
    use std::{collections::HashMap};
    use crate::api_client_service_impl::api_client_service_impl;


    #[derive(Serialize, Deserialize)]
    #[serde(crate = "rocket::serde")]
    pub struct JWTTokenResponse {
        pub access_token: String,
        pub token_type: String,
        pub expires_in: u16,
        pub refresh_token: String,
        pub scope: String,
        pub state: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Claims {
        aud: String,
        exp: usize,
        iat: usize,
        iss: String,
        nbf: usize,
        sub: String,
    }

    pub struct JWTErrorResponse {
        pub error_code: String,
        pub error_message: String,
    }

    pub async fn token(
        _client_id: &String,
        _client_secret: &String,
        _grant_type: &String,
    ) -> Result<JWTTokenResponse, JWTErrorResponse> {
        dotenv().ok();

        let mut app_base_url: String = "".to_string();

        match dotenvy::var("APP_BASE_URL") {
            Ok(value) => app_base_url = value,
            Err(err) => {
                tracing::warn!("error occurred while getting APP_BASE_URL: {}", err);
            }
        };

        let is_valid = is_valid(_client_id, _client_secret).await;

        if is_valid == false {
            return Err(JWTErrorResponse {
                error_code: 400.to_string(),
                error_message: String::from("invalid client id or client secret"),
            });
        }

        let mut header: Header = Header::new(jsonwebtoken::Algorithm::HS512);
        header.typ = Some("at+jwt".to_string());

        let mut extras = HashMap::with_capacity(1);
        extras.insert("custom".to_string(), "header".to_string());
        header.extras = extras;

        let current_date_time: DateTime<Utc> = Local::now().to_utc();
        let nbf = current_date_time.timestamp();

        let expires_on: DateTime<Utc> = current_date_time + TimeDelta::minutes(60);
        let exp: usize = expires_on.timestamp() as usize;

        let my_claims = Claims {
            aud: app_base_url.to_string(),
            exp: exp,
            iat: nbf as usize,
            iss: app_base_url.to_string(),
            nbf: nbf as usize,
            sub: _client_id.to_string(),
        };

        let token = encode(
            &header,
            &my_claims,
            &EncodingKey::from_secret("secret".as_ref()),
        )
        .unwrap();

        return Ok(JWTTokenResponse {
            access_token: token,
            token_type: "bearer".to_string(),
            expires_in: exp as u16,
            refresh_token: String::from(""),
            scope: String::from(""),
            state: String::from(""),
        });
    }

    async fn is_valid(_client_id: &String, _client_secret: &String) -> bool {
        let api_client_credential = api_client_service_impl::get_credential(_client_id).await;

        match api_client_credential {
            Ok(credential) => {

                let verify_client_secret = api_client_service_impl::verify_client_secret(_client_secret, &credential.client_secret).await;

                match verify_client_secret {
                    Ok(v) => {
                        return v;
                    }, 
                    Err(e) => {
                        tracing::warn!("error occurred while verify client secret: {}", e.error_message);
                        return false;
                    }
                }
            },
            Err(error) => {
                tracing::warn!("get credential error code: {} message: {}", error.error_code, error.error_message);
                return false;
            }
        }
    }
}
