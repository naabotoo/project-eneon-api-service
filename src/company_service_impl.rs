pub mod copmany_service_impl {

    use std::{env};

    use chrono::NaiveDateTime;
    use dotenvy::dotenv;
    use serde::{Deserialize, Serialize};
    use sqlx::{postgres::PgPoolOptions, prelude::FromRow};
    use uuid::Uuid;
    
    use crate::AddCompanyDTO;

    #[derive(Debug, FromRow)]
    struct RecordCompany {
        id: Uuid,
        name: String,
        email: String,
        subscription_tier: String,
        max_products: i32,
        max_categories: i32,
        is_enabled: bool,
        operates_in: Uuid,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(crate = "rocket::serde")]
    pub struct CompanyDTO {
        id: String,
        name: String,
        email: String,
        subscription_tier: String,
        max_products: usize,
        max_categories: usize,
        is_enabled: bool,
        operates_in: String,
        created_at: String,
        updated_at: String,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(crate = "rocket::serde")]
    pub struct CompanyDTOError {
        pub error_code: String,
        pub error_message: String,
    }

    pub struct CompanyDTOWithCount {
        pub data: Vec<CompanyDTO>,
        pub total_count: usize
    }

    //create company
    pub async fn create(add_company_request: AddCompanyDTO, subject: &String) -> Result<CompanyDTO, CompanyDTOError>{
        let db_connection = get_db_connection().await;

        match db_connection {
            Ok(conn) => {
                let id: Uuid = Uuid::new_v4();

                let statment = format!("INSERT into companies(id, name, email, is_enabled, operates_in) values($1, $2, $3, $4, $5)");

                let result = sqlx::query_as::<_, RecordCompany>(sqlx::AssertSqlSafe(statment))
                .bind(id)
                .bind(add_company_request.name)
                .bind(add_company_request.email)
                .bind(add_company_request.is_enabled)
                .bind(add_company_request.operates_in)
                .fetch_one(&conn).await;

                match result {
                    Ok(res) => {
                        return Ok(CompanyDTO { 
                            id: res.id.to_string(), 
                            name: res.name, 
                            email: res.email, 
                            subscription_tier: res.subscription_tier, 
                            max_products: res.max_products as usize, 
                            max_categories: res.max_categories as usize, 
                            is_enabled: res.is_enabled, 
                            operates_in: res.operates_in.to_string(), 
                            created_at: res.created_at.to_string(), 
                            updated_at: res.updated_at.to_string()
                        });
                    },
                    Err(err)=> {
                        tracing::warn!("error occurred while inserting create company by subject: {}. message: {}", subject, err.to_string());
                        return Err(CompanyDTOError {
                            error_code: 500.to_string(),
                            error_message: err.to_string(),
                        });
                    }
                }
            },
            Err(err) => {
                tracing::warn!("error occurred while getting db connection for create company subject: {}. message: {}", subject, err.to_string());
                return Err(CompanyDTOError {
                    error_code: 500.to_string(),
                    error_message: err.to_string(),
                });
            }
        }
    }

    //get company by id
    pub async fn get_by_id(id: &Uuid) -> Result<CompanyDTO, CompanyDTOError> {
        let db_connection = get_db_connection().await;

        match db_connection {
            Ok(conn) => {
                let statement = format!(
                    "SELECT c.id, 
                c.name,
                c. email, 
                c.subscription_tier, 
                c.max_products,
                c.max_categories, 
                c.is_enabled, 
                c.operates_in, 
                c.created_at, 
                c.updated_at FROM company AS c WHERE c.id = '{id}'"
                );

                let result = sqlx::query_as::<_, RecordCompany>(sqlx::AssertSqlSafe(statement))
                    .bind(id)
                    .fetch_one(&conn)
                    .await;

                match result {
                    Ok(res) => {
                        return Ok(CompanyDTO { id: res.id.to_string(), 
                            name: res.name, 
                            email: res.email, 
                            subscription_tier: res.subscription_tier, 
                            max_products: res.max_products as usize, 
                            max_categories: res.max_categories as usize, 
                            is_enabled: res.is_enabled, 
                            operates_in: res.operates_in.to_string(), 
                            created_at: res.created_at.to_string(), 
                            updated_at: res.updated_at.to_string() 
                        });
                    }
                    Err(err) => {
                        tracing::warn!("error occurred while getting company by id: {}. message: {}", id, err.to_string());
                        return Err(CompanyDTOError {
                            error_code: 400.to_string(),
                            error_message: err.to_string(),
                        });
                    }
                }
            },
            Err(err) => {
                tracing::warn!("error occurred while getting db connection for company by id: {}. message: {}", id, err.to_string());
                return Err(CompanyDTOError {
                    error_code: 500.to_string(),
                    error_message: err.to_string(),
                });
            }
        }
    }

    //get companies with pagination, search and filter
    pub async fn get_companies_by_filters(offset: &usize, limit: &usize, search: &String, subject: &String) -> Result<CompanyDTOWithCount, CompanyDTOError> {
        let db_connection = get_db_connection().await;

        match db_connection {
            Ok(conn) => {
                let statement = format!(
                    "SELECT c.id, 
                c.name,
                c. email, 
                c.subscription_tier, 
                c.max_products,
                c.max_categories, 
                c.is_enabled, 
                c.operates_in, 
                c.created_at, 
                c.updated_at FROM company AS c OFFSET {offset} LIMIT {limit}"
                );

                let result = sqlx::query_as::<_, RecordCompany>(sqlx::AssertSqlSafe(statement))
                    .bind(offset.clone() as i32)
                    .bind(limit.clone() as i32)
                    .fetch_all(&conn)
                    .await;

                match result {
                    Ok(res) => {
                        let mut found_data: Vec<CompanyDTO> = Vec::with_capacity(limit.clone());

                        for row in res {
                            let dto = CompanyDTO { 
                                id: row.id.to_string(), 
                                name: row.name, 
                                email: row.email, 
                                subscription_tier: row.subscription_tier, 
                                max_products: row.max_products as usize, 
                                max_categories: row.max_categories as usize, 
                                is_enabled: row.is_enabled, 
                                operates_in: row.operates_in.to_string(), 
                                created_at: row.created_at.to_string(), 
                                updated_at: row.updated_at.to_string() 
                            };

                            found_data.push(dto);

                        }

                        return Ok(CompanyDTOWithCount { data: found_data, total_count: 0 });

                    }
                    Err(err) => {
                        tracing::warn!("error occurred while getting companies by subject: {} with filters. message: {}", subject, err.to_string());
                        return Err(CompanyDTOError {
                            error_code: 400.to_string(),
                            error_message: err.to_string(),
                        });
                    }
                }

            },
            Err(err) => {
                tracing::warn!("error occurred while getting db connection for companies by subject: {} with filters. message: {}", subject, err.to_string());
                return Err(CompanyDTOError {
                    error_code: 500.to_string(),
                    error_message: err.to_string(),
                });
            }
        }
    }

    //update an existing company by id

    //delete an existing company by id

    //patch company is enabled status

    async fn get_db_connection() -> Result<sqlx::PgPool, sqlx::Error> {
        dotenv().ok();

        let host = env::var("DATABASE_HOST").unwrap();
        let port = env::var("DATABASE_PORT").unwrap();
        let username = env::var("DATABASE_USER").unwrap();
        let password = env::var("DATABASE_PASSWORD").unwrap();
        let db_name = env::var("DATABASE_NAME").unwrap();

        let max: u32 = 5;
        let url: String = format!(
            "postgres://{}:{}@{}:{}/{}",
            String::from(username),
            String::from(password),
            String::from(host),
            String::from(port),
            String::from(db_name)
        );

        return PgPoolOptions::new()
            .max_connections(max)
            .connect(&url.to_string())
            .await;
    }
}
