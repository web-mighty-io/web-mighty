DROP TABLE IF EXISTS pre_users;

CREATE TABLE pre_users
(
    id       VARCHAR(255) UNIQUE NOT NULL,
    email    VARCHAR(255) UNIQUE NOT NULL,
    token    UUID      DEFAULT gen_random_uuid(),
    gen_time TIMESTAMP DEFAULT now()
);

CREATE UNIQUE INDEX pre_users_id_index ON pre_users (id);
CREATE UNIQUE INDEX pre_users_email_index ON pre_users (email);
CREATE UNIQUE INDEX pre_users_token_index ON pre_users (token);

DROP TABLE IF EXISTS users;

CREATE SEQUENCE users_number_seq;

CREATE TABLE users
(
    no       INTEGER UNIQUE      NOT NULL DEFAULT nextval('users_number_seq'),
    id       VARCHAR(255) UNIQUE NOT NULL,
    name     VARCHAR(255)        NOT NULL,
    email    VARCHAR(255) UNIQUE NOT NULL, -- primary email
    rating   INT                          DEFAULT 0,
    password CHAR(255)           NOT NULL, -- hashed password
    gen_time TIMESTAMP                    DEFAULT now(),
    is_admin BOOLEAN                      DEFAULT FALSE
);

ALTER SEQUENCE users_number_seq OWNED BY users.no;
ALTER SEQUENCE users_number_seq RESTART WITH 10;

CREATE UNIQUE INDEX users_number_index ON users (no);
CREATE UNIQUE INDEX users_id_index ON users (id);
CREATE INDEX users_name_index ON users (name);
CREATE UNIQUE INDEX users_email_index ON users (email);
