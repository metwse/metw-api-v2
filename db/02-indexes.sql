-- Index to speed up lookups by username
CREATE INDEX IF NOT EXISTS users_username_id_idx
    ON users USING btree
    (username COLLATE pg_catalog."default" ASC NULLS LAST)
    INCLUDE(id)
    WITH (fillfactor=100, deduplicate_items=True);
