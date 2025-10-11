-- 1 "users" table
-- ------------------------------------------------
CREATE TABLE IF NOT EXISTS users
(
    id bigint NOT NULL,
    username character varying(20) COLLATE pg_catalog."default" NOT NULL,
    password text COLLATE pg_catalog."default" NOT NULL,
    flags bit(2) NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY (id),
    CONSTRAINT users_username_key UNIQUE (username)
);

-- 2 "emails" table
-- depends on: "users"
-- ------------------------------------------------
CREATE TABLE IF NOT EXISTS emails
(
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    email text COLLATE pg_catalog."default" NOT NULL,
    is_verified boolean NOT NULL DEFAULT false,
    CONSTRAINT emails_pkey PRIMARY KEY (id),
    CONSTRAINT emails_email_key UNIQUE (email),
    CONSTRAINT emails_user_id_fkey FOREIGN KEY (user_id)
        REFERENCES users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

-- 3 "threads" table
-- depends on: "users"
-- ------------------------------------------------
CREATE TABLE IF NOT EXISTS threads
(
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    CONSTRAINT threads_pkey PRIMARY KEY (id),
    CONSTRAINT threads_user_id_fkey FOREIGN KEY (user_id)
        REFERENCES users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID
);

-- 4 "profiles" table
-- depends on: "users", "threads"
-- ------------------------------------------------
CREATE TABLE IF NOT EXISTS profiles
(
    user_id bigint NOT NULL,
    comments_thread_id bigint NOT NULL,
    avatar_id bigint,
    banner_id bigint,
    bio text COLLATE pg_catalog."default",
    CONSTRAINT profiles_pkey PRIMARY KEY (user_id),
    CONSTRAINT profiles_thread_id_fkey FOREIGN KEY (comments_thread_id)
        REFERENCES threads (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID,
    CONSTRAINT profiles_user_id_fkey FOREIGN KEY (user_id)
        REFERENCES users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

-- 5 "posts" table
-- depends on: "users", "threads"
-- ------------------------------------------------
CREATE TABLE IF NOT EXISTS posts
(
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    thread_id bigint,
    replies_thread_id bigint NOT NULL,
    content text COLLATE pg_catalog."default",
    is_edited boolean NOT NULL DEFAULT false,
    attachments bigint[],
    CONSTRAINT posts_pkey PRIMARY KEY (id),
    CONSTRAINT posts_replies_thread_id_fkey FOREIGN KEY (replies_thread_id)
        REFERENCES threads (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT posts_thread_id_fkey FOREIGN KEY (thread_id)
        REFERENCES threads (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT posts_user_id_fkey FOREIGN KEY (user_id)
        REFERENCES users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);
