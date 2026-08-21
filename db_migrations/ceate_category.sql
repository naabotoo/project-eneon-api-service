CREATE TABLE Category (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    parent_category_id UUID,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_on TIMESTAMP DEFAULT NOW(),
    is_active BOOLEAN DEFAULT TRUE,
    FOREIGN KEY (company_id) REFERENCES Company(id),
    FOREIGN KEY (parent_category_id) REFERENCES Category(id),
    UNIQUE (company_id, name)
);

CREATE INDEX idx_company_id ON Category(company_id);