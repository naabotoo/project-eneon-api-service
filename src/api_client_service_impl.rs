pub mod api_client_service_impl {
use std::{ env, str::FromStr };

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{ SaltString, rand_core::OsRng}};
use chrono::{ NaiveDateTime };
use rand::{RngExt, distr::Alphanumeric};
use serde::{Deserialize, Serialize };
use sqlx::{AssertSqlSafe, postgres::PgPoolOptions, prelude::FromRow};
use dotenvy::dotenv;
use uuid::Uuid;

    #[derive(Serialize, Deserialize)]
    #[serde(crate = "rocket::serde")]
    pub struct ApiClientCredential {
        pub id: String,
        pub client_id: String,
        pub client_secret: String,
        pub is_active: bool,
        pub created_on: String,
        pub updated_on: String,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(crate = "rocket::serde")]
    pub struct ApiClientCredentialError {
        pub error_code: String,
        pub error_message: String,
    }

    #[derive(Debug, FromRow)]
    pub struct RecordApiClientCredential {
        pub id: Uuid,
        pub client_id: String,
        pub client_secret: String,
        pub is_active: bool,
        pub created_on: NaiveDateTime,
        pub updated_on: NaiveDateTime,
    }

    #[derive(Debug, FromRow)]
    pub struct RecordCreateApiClientCredential {
        pub _id: String
    }

    #[derive(Debug)]
    pub struct GeneratedClientSecret {
        pub client_secret: String
    }

    #[derive(Debug)]
    pub struct GeneratedClientSecretError {
        pub error: String
    }

    #[derive(Debug)]
    pub struct EncryptedClientSecret {
        pub encrypted_secret: String
    } 
    
    #[derive(Debug)]
    pub struct EncryptedClientSecretError {
        pub error_message: String
    }

    #[derive(Debug)]
    pub struct VerifyClientSecretError {
        pub error_message: String
    }

    #[derive(Debug)]
    pub struct IsClientValidError {
        pub error_code: String,
        pub error_message: String
    }

    #[derive(Debug, FromRow)]
    pub struct RecordIsClientValid {
        is_allowed: bool
    }
    pub async fn create_credential(
        _client_id: &String,
        _client_secret: &String,
        _is_active: &bool
    ) -> Result<ApiClientCredential, ApiClientCredentialError> {

        tracing::info!("create credential with client id: {}", _client_id);

        let db_connection = get_db_connection().await;

        match db_connection {
            Ok(pool) => {
                let mut _id: Uuid = Uuid::nil();

                let statement = format!("SELECT * FROM create_api_client_credentials('{}', '{}', {}) AS _id;", _client_id, _client_secret, _is_active);
                
                let result = sqlx::query_as::<_, RecordCreateApiClientCredential>(AssertSqlSafe(statement))
                .bind(&_id)
                .fetch_one(&pool)
                .await;

                match result {
                    Ok(r)  => {
                        let saved: Result<ApiClientCredential, ApiClientCredentialError> = get_by_id(Uuid::from_str(r._id.as_str()).unwrap().as_ref()).await;

                        match saved {
                            Ok(r) => {
                                return Ok(r);
                            },
                            Err(e) => {
                                return Err(ApiClientCredentialError { 
                                    error_code: e.error_code.to_string(), 
                                    error_message: e.error_message.to_string() 
                                });
                            }
                        };
                    },
                    Err(err) => {
                        return Err(ApiClientCredentialError { error_code: 500.to_string(), error_message: err.to_string() });
                    }
                }
            
            },
            Err(err) => {
                return Err(ApiClientCredentialError { error_code: 500.to_string(), error_message: err.to_string() });
            }
        }
    }

    pub async fn get_credential(
        _client_id: &String,
    ) -> Result<ApiClientCredential, ApiClientCredentialError> {

        let db_connection = get_db_connection().await;

        match db_connection {
            Ok(conn) => {
                let statement = format!("SELECT acc.id, acc.client_id, acc.client_secret, acc.is_active, acc.created_on, acc.updated_on FROM api_client_credentials AS acc WHERE acc.client_id LIKE '%{_client_id}%'");

                let result = sqlx::query_as::<_, RecordApiClientCredential>(sqlx::AssertSqlSafe(statement))
                .bind(_client_id)
                .fetch_one(&conn)
                .await;

                match result {
                    Ok(row) => {

                        if row.is_active == false {
                            return Err(ApiClientCredentialError { 
                                error_code: "400".to_string(), 
                                error_message: "inactive client id".to_string()
                            });
                        }

                        return Ok( ApiClientCredential {
                            id: row.id.to_string(),
                            client_id: row.client_id,
                            client_secret: row.client_secret,
                            is_active: row.is_active,
                            created_on: row.created_on.to_string(),
                            updated_on: row.updated_on.to_string()
                        });
                    },
                    Err(err) => {
                        tracing::warn!("error : {}", err);

                        return Err(ApiClientCredentialError { error_code: 
                        "400".to_string(), 
                        error_message: err.to_string() 
                })
                    }
                }

            },
            Err(err) => {
                println!("error : {}", err);

                return Err(ApiClientCredentialError { error_code: 
                    "500".to_string(), 
                    error_message: err.to_string() 
                })
            }
        };

    }

    pub async fn get_by_id (
        _id: &Uuid,
    ) -> Result<ApiClientCredential, ApiClientCredentialError> {

        let db_connection = get_db_connection().await;

        match db_connection {
            Ok(conn) => {
                let statement = format!("SELECT acc.id, acc.client_id, acc.client_secret, acc.is_active, acc.created_on, acc.updated_on FROM api_client_credentials AS acc WHERE acc.id = '{_id}'");

                let result = sqlx::query_as::<_, RecordApiClientCredential>(sqlx::AssertSqlSafe(statement))
                .bind(_id)
                .fetch_one(&conn)
                .await;

                match result {
                    Ok(row) => {

                        if row.is_active == false {
                            return Err(ApiClientCredentialError { 
                                error_code: "400".to_string(), 
                                error_message: "inactive client id".to_string()
                            });
                        }

                        return Ok( ApiClientCredential {
                            id: row.id.to_string(),
                            client_id: row.client_id,
                            client_secret: row.client_secret,
                            is_active: row.is_active,
                            created_on: row.created_on.to_string(),
                            updated_on: row.updated_on.to_string()
                        });
                    },
                    Err(err) => {
                        println!("error : {}", err);

                        return Err(ApiClientCredentialError { error_code: 
                        "400".to_string(), 
                        error_message: err.to_string() 
                })
                    }
                }

            },
            Err(err) => {
                return Err(ApiClientCredentialError { error_code: 
                    "500".to_string(), 
                    error_message: err.to_string() 
                });
            }
        };

    }

    pub async fn delete_credential(_id: &Uuid) -> Result<ApiClientCredential, ApiClientCredentialError> {
        let db_connection = get_db_connection().await;

        match db_connection {
            Ok(pool) => {
                let statement = format!("DELETE FROM api_client_credentials WHERE id='{}'", _id);

                let result = sqlx::raw_sql(sqlx::AssertSqlSafe(statement))
                .execute(&pool)
                .await;

                match result {
                    Ok(r) => {

                        if r.rows_affected() > 0 {
                            return Ok(ApiClientCredential { id: _id.to_string(), 
                                client_id: "".to_string(), 
                                client_secret: "".to_string(), 
                                is_active: false, 
                                created_on: "".to_string(), 
                                updated_on: "".to_string() 
                            });
                        } else {
                            return Err(ApiClientCredentialError { 
                                error_code: 404.to_string(), 
                                error_message: "invalid api client credentials".to_string() 
                            });
                        }
                    },
                    Err(e)=> {
                        return Err(ApiClientCredentialError { error_code: 500.to_string(), error_message: e.to_string() });
                    }
                }
            },
            Err(e) => {
                return Err(ApiClientCredentialError { error_code: 
                    "500".to_string(), 
                    error_message: e.to_string() 
                });
            }
        };
    }

    // pub fn update_credential() -> Result<<Api_Client_Credential, Api_Client_Credential_Error> {

    // }

    // pub fn set_is_active() -> Result<Api_Client_Credential, Api_Client_Credential_Error> {
 
    // }

    pub async fn get_api_client_credentials(search: &str, offset: i32, limit: i32) -> Result<Vec<ApiClientCredential>, ApiClientCredentialError> {
        let db_connection = get_db_connection().await;

        match db_connection {
            Ok(pool) => {
                let mut statement = format!("SELECT acc.id, acc.client_id, acc.client_secret, acc.is_active, acc.created_on, acc.updated_on FROM api_client_credentials AS acc LIMIT {limit} OFFSET {offset}");

                if search.is_empty() == false {
                    statement = format!("SELECT acc.id, acc.client_id, acc.client_secret, acc.is_active, acc.created_on, acc.updated_on FROM api_client_credentials AS acc WHERE acc.client_id LIKE '%{search}%' LIMIT {limit} OFFSET {offset}");
                }

                let result = sqlx::query_as::<_, RecordApiClientCredential>(sqlx::AssertSqlSafe(statement))
                .bind(offset)
                .bind(limit)
                .fetch_all(&pool)
                .await;
                
                match result {
                    Ok(records) => {
                        let mut data: Vec<ApiClientCredential> = Vec::with_capacity(limit as usize);

                        for record in records.iter() {

                            let client_credential = ApiClientCredential {
                                id: record.id.to_string(),
                                client_id: record.client_id.to_string(),
                                client_secret: record.client_secret.to_string(),
                                is_active: record.is_active,
                                created_on: record.created_on.to_string(),
                                updated_on: record.updated_on.to_string()
                            };

                            data.push(client_credential);
                        }

                        return Ok(data);
                    }, 
                    Err(err) => {
                        return Err(ApiClientCredentialError { error_code: 
                            "500".to_string(), 
                            error_message: err.to_string() 
                        });
                    }
                }
                
            },
            Err(err) => {
                return Err(ApiClientCredentialError { error_code: 
                    "500".to_string(), 
                    error_message: err.to_string() 
                });
            }
        }
    }

    pub async fn generate_client_secret() -> Result<GeneratedClientSecret, GeneratedClientSecretError> {
        let random_string: String = rand::rng().sample_iter(&Alphanumeric).take(32).map(char::from).collect();
        return Ok(GeneratedClientSecret { client_secret: random_string });
    }

    pub async fn encrypt_client_secret(client_secret: &String) -> Result<EncryptedClientSecret, EncryptedClientSecretError> {

        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(client_secret.as_bytes(), &salt);

        match password_hash {
            Ok(password_hash) => {
                return Ok(EncryptedClientSecret { encrypted_secret: password_hash.to_string() });
            },
            Err(err) => {
                return Err(EncryptedClientSecretError {  error_message: err.to_string() });
            }
        }
    }

    pub async fn generate_and_encrypt_client_secret() -> Result<EncryptedClientSecret, EncryptedClientSecretError>{
        let generate_client_secret = generate_client_secret().await;

        match generate_client_secret {
            Ok(secret) => {
                let secret_value = secret.client_secret;

                let encrypted_value = encrypt_client_secret(&secret_value).await;

                match encrypted_value {
                    Ok(encrypted) => {
                        return Ok(encrypted);
                    },
                    Err(e) => {
                        return Err(EncryptedClientSecretError { error_message: e.error_message });
                    }
                }
            },
            Err(err) => {
                return Err(EncryptedClientSecretError { error_message: err.error });
            }
        }

    }

    pub async fn verify_client_secret(_incoming_client_secret: &String, _stored_client_secret: &String) -> Result<bool, VerifyClientSecretError> {
        let argon2 = Argon2::default();
        let hashed_password = PasswordHash::new(_stored_client_secret);

        match hashed_password {
            Ok(hash) => {
                let is_valid = argon2.verify_password(_incoming_client_secret.as_bytes(), &hash).is_ok();

                return Ok(is_valid);
            },
            Err(e) => {
                return Err(VerifyClientSecretError { error_message: e.to_string() });
            }
        }
        
    }

    pub async fn is_allowed(client_id: &str) -> Result<bool, IsClientValidError> {
        let db_connection = get_db_connection().await;

        match db_connection {
            Ok(pool) => {
                let check_query = format!("SELECT is_client_allowed('{client_id}') AS is_allowed;");

                let result = sqlx::query_as::<_, RecordIsClientValid>(sqlx::AssertSqlSafe(check_query))
                .bind(client_id)
                .fetch_one(&pool)
                .await;

                match result {
                    Ok(r) => {
                        return Ok(r.is_allowed)
                    },
                    Err(e) => {
                        println!("error occurred : {}", e);
                        return Err(IsClientValidError { 
                            error_code: 500.to_string(), 
                            error_message: e.to_string() 
                        });
                    }
                }
            },
            Err(err) => {
                return Err(IsClientValidError { 
                    error_code: "500".to_string(), 
                    error_message: err.to_string() 
                });
            }
        }
    }

    async fn get_db_connection() -> Result<sqlx::PgPool, sqlx::Error> {
        dotenv().ok();
        
        let host = env::var("DATABASE_HOST").unwrap();
        let port = env::var("DATABASE_PORT").unwrap();
        let username = env::var("DATABASE_USER").unwrap();
        let password = env::var("DATABASE_PASSWORD").unwrap();
        let db_name = env::var("DATABASE_NAME").unwrap();

        let max: u32 = 5;
        let url: String = format!("postgres://{}:{}@{}:{}/{}", String::from(username), String::from(password), String::from(host), String::from(port), String::from(db_name));

        return PgPoolOptions::new().max_connections(max).connect(&url.to_string()).await;
    }

}
