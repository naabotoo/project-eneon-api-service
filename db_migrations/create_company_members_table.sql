CREATE TABLE company_member_assigned_areas (
    id UUID PRIMARY KEY gen_random_uuid(),
    company_member_id UUID NOT NULL,
    area_id UUID NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_on TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_on TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_company_member_assigned_areas_company_member
        FOREIGN KEY (company_member_id)
        REFERENCES company_members (id),

    CONSTRAINT fk_company_member_assigned_areas_area
        FOREIGN KEY (area_id)
        REFERENCES areas (id),

    CONSTRAINT uq_company_member_assigned_area
        UNIQUE (company_member_id, area_id)
);
