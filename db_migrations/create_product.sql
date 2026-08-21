CREATE TABLE Product (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id UUID NOT NULL,
    category_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    sku VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    is_active BOOLEAN DEFAULT TRUE,
    FOREIGN KEY (company_id) REFERENCES Company(id)
);

CREATE INDEX idx_company_id ON Product(company_id);
CREATE INDEX idx_prod_category_id ON Product(company_id, category_id);
ALTER TABLE Product ADD UNIQUE (category_id, sku);
ALTER TABLE product ADD FOREIGN KEY (category_id) REFERENCES category(id);
ALTER TABLE product ADD FOREIGN KEY (company_id) REFERENCES company(id);