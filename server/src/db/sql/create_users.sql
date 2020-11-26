CREATE TABLE users (
    no       SERIAL       PRIMARY KEY,
    id       VARCHAR(255) UNIQUE NOT NULL,
    name     VARCHAR(255) NOT NULL,
    email    VARCHAR(255) UNIQUE NOT NULL,
    email_v  BOOLEAN      DEFAULT FALSE,
    rating   INT          DEFAULT 0,
    password VARCHAR(255) NOT NULL
);