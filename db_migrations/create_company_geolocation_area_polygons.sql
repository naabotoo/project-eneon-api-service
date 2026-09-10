CREATE TABLE company_geolocation_area_polygons (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    area_id UUID,
    idx INT NOT NULL,
    latitude VARCHAR(10) NOT NULL,
    longitude VARCHAR(10) NOT NULL,
    created_on TIMESTAMP DEFAULT NOW(),
    updated_on TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (area_id) REFERENCES company_geolocation_areas (id)
);
