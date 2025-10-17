-- Index to speed up lookups by username
CREATE INDEX IF NOT EXISTS users_username_id_idx
    ON users USING btree
    (username COLLATE pg_catalog."default" ASC NULLS LAST)
    INCLUDE(id)
    WITH (fillfactor=100, deduplicate_items=True);

-- Index to speed up post lookups by thread_id
CREATE INDEX IF NOT EXISTS posts_thread_id_id_idx
    ON posts USING btree
    (thread_id ASC NULLS FIRST)
    INCLUDE(id)
    WITH (fillfactor=100, deduplicate_items=True);

-- Index to speed up post lookups by replies_thread_id
CREATE INDEX IF NOT EXISTS posts_replies_thread_id_id_idx
    ON public.posts USING btree
    (replies_thread_id ASC NULLS LAST)
    INCLUDE(id)
    WITH (fillfactor=100, deduplicate_items=True);

