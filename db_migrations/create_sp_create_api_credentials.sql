CREATE OR REPLACE FUNCTION create_api_client_credentials(
  IN p_client_id TEXT,
  IN p_client_secret TEXT,
  IN p_is_active BOOLEAN DEFAULT FALSE
)
RETURNS TEXT
LANGUAGE plpgsql
AS $$
DECLARE
  v_id TEXT;
BEGIN
  INSERT INTO api_client_credentials (client_id, client_secret, is_active)
  VALUES (p_client_id, p_client_secret, COALESCE(p_is_active, FALSE))
  RETURNING id INTO v_id;

  RETURN v_id;
END;
$$;