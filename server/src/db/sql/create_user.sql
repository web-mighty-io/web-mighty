CREATE TABLE pre_users
(
    id             VARCHAR(255) UNIQUE NOT NULL,
    email          VARCHAR(255) UNIQUE NOT NULL,
    token          UUID      DEFAULT UUID_GENERATE_V4(),
    generated_time TIMESTAMP DEFAULT NOW()
);

CREATE UNIQUE INDEX pre_users_id_index ON pre_users (id);
CREATE UNIQUE INDEX pre_users_email_index ON pre_users (email);
CREATE UNIQUE INDEX pre_users_token_index ON pre_users (token);

CREATE SEQUENCE users_number_seq;

CREATE TABLE users
(
    no              INTEGER UNIQUE      NOT NULL DEFAULT NEXTVAL('users_number_seq'),
    id              VARCHAR(255) UNIQUE NOT NULL,
    username        VARCHAR(255)        NOT NULL,
    email           VARCHAR(255) UNIQUE NOT NULL, -- primary email
    rating          INT                          DEFAULT 0,
    password        CHAR(255)           NOT NULL, -- hashed password
    registered_time TIMESTAMP                    DEFAULT NOW(),
    is_admin        BOOLEAN                      DEFAULT FALSE
);

ALTER SEQUENCE users_number_seq OWNED BY users.no;
ALTER SEQUENCE users_number_seq RESTART WITH 10;

CREATE UNIQUE INDEX users_number_index ON users (no);
CREATE UNIQUE INDEX users_id_index ON users (id);
CREATE INDEX users_name_index ON users (username);
CREATE UNIQUE INDEX users_email_index ON users (email);

CREATE TABLE email
(
    no             INTEGER UNIQUE      NOT NULL,
    id             VARCHAR(255) UNIQUE NOT NULL,
    email          VARCHAR(255)        NOT NULL,
    verified       BOOLEAN   DEFAULT FALSE,
    token          UUID      DEFAULT UUID_GENERATE_V4(),
    generated_time TIMESTAMP DEFAULT NOW()
);

CREATE UNIQUE INDEX email_number_index ON email (no);
CREATE UNIQUE INDEX email_id_index ON email (id);
CREATE UNIQUE INDEX email_token_index ON email (token);