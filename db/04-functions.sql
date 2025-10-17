-- Generates a snowflake-like ID for NOW() without increment bytes.
CREATE OR REPLACE FUNCTION snowflake_like()
    RETURNS bigint
    LANGUAGE 'sql'
    VOLATILE LEAKPROOF PARALLEL SAFE
RETURN (((EXTRACT(epoch FROM now()))::bigint - 1660262400) * 1000) << 22;

-- Generates a snowflake-like ID for interval time ago without increment bytes.
CREATE FUNCTION snowflake_like_base_past(time_ago interval)
    RETURNS bigint
    LANGUAGE 'sql'
    VOLATILE LEAKPROOF PARALLEL SAFE
RETURN (((EXTRACT(epoch FROM (now() - time_ago)))::bigint - 1660262400) * 1000) << 22;
