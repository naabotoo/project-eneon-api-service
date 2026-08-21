CREATE OR REPLACE FUNCTION get_company_by_id_and_status(
    p_company_id UUID,
    p_is_enabled BOOLEAN
)
RETURNS TABLE (
    id UUID,
    company_id UUID,
    name VARCHAR(255),
    description TEXT,
    parent_category_id UUID,
    created_at TIMESTAMP,
    modified_at TIMESTAMP,
    is_active BOOLEAN
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        c.id,
        c.company_id,
        c.name,
        c.description,
        c.parent_category_id,
        c.created_at,
        c.modified_at,
        c.is_active   
    FROM category c
    WHERE c.company_id = p_company_id
    AND c.is_active = p_is_enabled;
END;
$$ LANGUAGE plpgsql;