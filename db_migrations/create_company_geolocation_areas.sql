CREATE TABLE company_geolocation_areas (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    country_id UUID,
    company_id UUID,
    label VARCHAR(255) NOT NULL,
    code VARCHAR(10) NOT NULL UNIQUE,
    description TEXT NULL,
    is_active BOOLEAN,
    created_on TIMESTAMP DEFAULT NOW(),
    updated_on TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (country_id) REFERENCES supported_countries (id),
    FOREIGN KEY (company_id) REFERENCES company (id)
);
