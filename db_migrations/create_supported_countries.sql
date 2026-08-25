CREATE TABLE supported_countries (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    iso_code VARCHAR NOT NULL,
    is_enabled BOOLEAN DEFAULT TRUE,
    created_on TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_on TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE supported_languages (
    id UUID PRIMARY KEY,
    label VARCHAR NOT NULL,
    iso_code VARCHAR NOT NULL,
    created_on TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_on TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE supported_currencies (
    id UUID PRIMARY KEY,
    label VARCHAR NOT NULL,
    iso_code VARCHAR NOT NULL,
    created_on TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_on TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE supported_country_language (
    id UUID PRIMARY KEY,
    country_id UUID NOT NULL,
    language_id UUID NOT NULL,
    created_on TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_on TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    CONSTRAINT fk_country_language_country
        FOREIGN KEY (country_id)
        REFERENCES supported_countries (id),

    CONSTRAINT fk_country_language_language
        FOREIGN KEY (language_id)
        REFERENCES supported_languages (id),

    CONSTRAINT uq_supported_country_language
        UNIQUE (country_id, language_id)
);

CREATE TABLE supported_country_currencies (
    id UUID PRIMARY KEY,
    country_id UUID NOT NULL,
    currency_id UUID NOT NULL,
    created_on TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_on TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    CONSTRAINT fk_country_currency_country FOREIGN KEY (country_id) REFERENCES supported_countries (id),
    CONSTRAINT fk_country_currency_currency FOREIGN KEY (currency_id) REFERENCES supported_currencies (id),
    CONSTRAINT uq_supported_country_currency UNIQUE (country_id, currency_id)
);


CREATE INDEX idx_supported_country_language_country_id
    ON supported_country_language (country_id);

CREATE INDEX idx_supported_country_language_language_id
    ON supported_country_language (language_id);

CREATE INDEX idx_supported_country_currencies_country_id
    ON supported_country_currencies (country_id);

CREATE INDEX idx_supported_country_currencies_currency_id
    ON supported_country_currencies (currency_id);


ALTER TABLE supported_countries
    ADD CONSTRAINT uq_supported_countries_iso_code
    UNIQUE (iso_code);

ALTER TABLE supported_languages
    ADD CONSTRAINT uq_supported_languages_iso_code
    UNIQUE (iso_code);

ALTER TABLE supported_currencies
    ADD CONSTRAINT uq_supported_currencies_iso_code
    UNIQUE (iso_code);
