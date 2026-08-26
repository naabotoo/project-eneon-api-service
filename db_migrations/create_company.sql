CREATE TABLE Company (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    subscription_tier VARCHAR(50) DEFAULT 'BASIC',
    max_products INT DEFAULT 1000,
    max_categories INT DEFAULT 100,
    is_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    operates_in UUID NOT NULL references supported_countries(id),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    is_active BOOLEAN DEFAULT TRUE
);