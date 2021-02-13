CREATE TABLE IF NOT EXISTS pre_users
(
    id       VARCHAR(31) UNIQUE NOT NULL,
    email    VARCHAR(63) UNIQUE NOT NULL,
    token    CHAR(64)           NOT NULL,
    gen_time TIMESTAMP DEFAULT now()
);

CREATE UNIQUE INDEX IF NOT EXISTS pre_users_id_index ON pre_users (id);
CREATE UNIQUE INDEX IF NOT EXISTS pre_users_email_index ON pre_users (email);
CREATE UNIQUE INDEX IF NOT EXISTS pre_users_token_index ON pre_users (token);

CREATE SEQUENCE IF NOT EXISTS users_number_seq;

CREATE TABLE IF NOT EXISTS users
(
    no       INTEGER UNIQUE      NOT NULL DEFAULT nextval('users_number_seq'),
    id       VARCHAR(31) UNIQUE NOT NULL,
    name     VARCHAR(63)        NOT NULL,
    email    VARCHAR(63) UNIQUE NOT NULL, -- primary email
    rating   INT                          DEFAULT 0,
    password CHAR(128)           NOT NULL, -- hashed password
    gen_time TIMESTAMP                    DEFAULT now(),
    is_admin BOOLEAN                      DEFAULT FALSE
);

ALTER SEQUENCE users_number_seq OWNED BY users.no;
ALTER SEQUENCE users_number_seq RESTART WITH 100;

CREATE UNIQUE INDEX IF NOT EXISTS users_number_index ON users (no);
CREATE UNIQUE INDEX IF NOT EXISTS users_id_index ON users (id);
CREATE INDEX IF NOT EXISTS users_name_index ON users (name);
CREATE UNIQUE INDEX IF NOT EXISTS users_email_index ON users (email);
