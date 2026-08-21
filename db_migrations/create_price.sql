CREATE TYPE price_type_enum AS ENUM (
    'RETAIL',
    'WHOLESALE',
    'SALE',
    'COST',
    'DISTRIBUTOR',
    'BUNDLE',
    'CLEARANCE',
    'PROMOTIONAL'
);

CREATE TABLE Price (
    price_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id UUID NOT NULL,
    product_id UUID NOT NULL,
    price_amount DECIMAL(10, 2) NOT NULL,
    currency VARCHAR(3) DEFAULT 'GHS',
    effective_date DATE NOT NULL,
    end_date DATE,
    price_type price_type_enum NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    modified_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (company_id) REFERENCES Company(id),
    FOREIGN KEY (product_id) REFERENCES Product(id)  
);

CREATE INDEX idx_company_product ON Price(company_id, product_id);
CREATE INDEX idx_effective_date ON Price(company_id, effective_date, end_date);