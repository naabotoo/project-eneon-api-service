pub mod geofencing_services_impl {
    use std::str::FromStr;

use uuid::Uuid;

use crate::{WithinFenceRequestFilter, WithinFenceResult, company_service_impl};

    pub struct WithinFenceResultError {
        pub error_code: String,
        pub error_message: String
    }


    pub async fn is_within_fence(subject: &String, query: WithinFenceRequestFilter) -> Result<WithinFenceResult, WithinFenceResultError>{
        let company_member_id = query.company_member_id;

        let id_as_uuid = Uuid::from_str(&company_member_id);

        match id_as_uuid {
            Ok(id) => {
                let compony_member = company_service_impl::copmany_service_impl::find_company_member_by_id(id).await;

                match compony_member {

                    Ok(member) => {
                        let company = member.company;
                        
                        return Ok(WithinFenceResult {  });
                    },
                    Err(err) => {
                        tracing::warn!("is_within_fence: error occurred while getting compnay member by id: {}. message: {}", id, err.error_message);

                        return Err(WithinFenceResultError {
                            error_code: 400.to_string(),
                            error_message: "invalid company member id passed.".to_ascii_lowercase()
                        });
                    }
                }
            },
            Err(err) => {
                tracing::warn!("is_within_fence: error occurred while converting string to uuid. message: {}", err.to_string());

                return Err(WithinFenceResultError {
                    error_code: 400.to_string(),
                    error_message: err.to_string()
                });
            }
        }

    }
    
}