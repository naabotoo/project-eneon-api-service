CREATE TABLE sign_up_request (
    id UUID PRIMARY KEY,
    msisdn VARCHAR(50),
    email VARCHAR(255),
    country_id UUID,
    client_id UUID,
    is_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    is_expired BOOLEAN NOT NULL DEFAULT FALSE,
    created_on TIMESTAMP NOT NULL,
    updated_on TIMESTAMP NOT NULL,
    fk_sign_up_request_country FOREIGN KEY (country_id) REFERENCES supported_countires (id),
    fk_sign_up_request_client FOREIGN KEY (client_id) REFERENCES clients (id)
);
