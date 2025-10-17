CREATE SCHEMA IF NOT EXISTS relations;

-- 1 "likes" table
-- user_id likes -> post_id
-- ------------------------------------------------
CREATE TABLE IF NOT EXISTS relations.likes
(
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    post_id bigint NOT NULL,
    CONSTRAINT likes_id_pkey PRIMARY KEY (id),
    CONSTRAINT likes_post_id_fkey FOREIGN KEY (post_id)
        REFERENCES posts (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT likes_user_id_fkey FOREIGN KEY (user_id)
        REFERENCES users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

CREATE INDEX IF NOT EXISTS likes_post_id_user_id_idx
    ON relations.likes USING btree
    (post_id ASC NULLS LAST)
    INCLUDE(user_id)
    WITH (fillfactor=100, deduplicate_items=True);

CREATE INDEX IF NOT EXISTS likes_user_id_post_id_idx
    ON relations.likes USING btree
    (user_id ASC NULLS LAST)
    INCLUDE(post_id)
    WITH (fillfactor=100, deduplicate_items=True);

-- 2 "follows" table
-- follower_id follows -> user_id
-- ------------------------------------------------
CREATE TABLE IF NOT EXISTS relations.follows
(
    id bigint NOT NULL,
    follower_id bigint NOT NULL,
    user_id bigint NOT NULL,
    CONSTRAINT follows_id_pkey PRIMARY KEY (id),
    CONSTRAINT follows_follower_id_fkey FOREIGN KEY (follower_id)
        REFERENCES users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT follows_user_id_fkey FOREIGN KEY (user_id)
        REFERENCES users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

CREATE INDEX IF NOT EXISTS follows_follower_id_user_id_idx
    ON relations.follows USING btree
    (follower_id ASC NULLS LAST)
    INCLUDE(user_id)
    WITH (fillfactor=100, deduplicate_items=True);

CREATE INDEX IF NOT EXISTS follows_user_id_follower_id_idx
    ON relations.follows USING btree
    (user_id ASC NULLS LAST)
    INCLUDE(follower_id)
    WITH (fillfactor=100, deduplicate_items=True);
