pub mod categories_service_impl {
    use uuid::Uuid;
    use chrono::NaiveDateTime;
    use sqlx::{AssertSqlSafe, postgres::PgPoolOptions, prelude::FromRow};
    use std::{ env };
    use dotenvy::dotenv;

    #[derive(Debug, FromRow)]
    pub struct RecordProductCategory {
        pub id: Uuid,
        pub company_id: Uuid, 
        pub name: String,
        pub description: String,
        pub parent_category_id: Option<Uuid>,
        pub created_at: NaiveDateTime,
        pub modified_at: NaiveDateTime,
        pub is_active: bool  
    }

    pub struct ProductServiceError {
        pub error_code: i32,
        pub error_message: String
    }

    pub async fn get_categories_by_company_id(company_id: &Uuid, is_active: &bool) -> Result<Vec<RecordProductCategory>, ProductServiceError> {
        tracing::info!("get product categories by company id: {}", company_id);

        let db_connection = get_db_connection().await;


        match db_connection {
            Ok(pool) => {

                let statement = format!("SELECT * FROM get_company_by_id_and_status('{}', {});", company_id, is_active);
                
                let result = sqlx::query_as::<_, RecordProductCategory>(AssertSqlSafe(statement))
                .bind(&company_id)
                .bind(&is_active)
                .fetch_all(&pool)
                .await;

                match result {
                    Ok(res) => {
                        return Ok(res);
                    },
                    Err(e) => {
                        tracing::warn!("error while getting product category by company id from db. message {}", e);
                        return Err(ProductServiceError { error_code: 500, error_message: e.to_string() });
                    }
                }
            },
            Err(e) => {
                tracing::warn!("error while connecting to db for product categories by company id. message {}", e);
                return Err(ProductServiceError { error_code: 500, error_message: e.to_string() });
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