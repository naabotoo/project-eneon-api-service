CREATE TABLE company_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id UUID,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    email VARCHAR(100),
    msisdn VARCHAR(10),
    has_mfa_enabled BOOLEAN,
    is_active BOOLEAN,
    created_on TIMESTAMP DEFAULT NOW(),
    updated_on TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (company_id) REFERENCES company(id),
    UNIQUE (company_id, msisdn)
);
