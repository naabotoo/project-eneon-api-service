CREATE OR REPLACE FUNCTION is_client_allowed(p_client_id text)
RETURNS boolean
LANGUAGE sql
AS $$
  SELECT CASE
    WHEN COUNT(*) > 0 THEN true
    ELSE false
  END
  FROM api_client_credentials acc
  WHERE acc.is_active = true
    AND acc.client_id = p_client_id;
$$;