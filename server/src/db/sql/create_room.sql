CREATE TABLE IF NOT EXISTS curr_rooms
(
    uid       CHAR(64)    NOT NULL UNIQUE,
    id        INTEGER     NOT NULL UNIQUE,
    name      VARCHAR(63) NOT NULL,
    users_cnt    INTEGER   NOT NULL,
    is_gaming BOOLEAN     NOT NULL DEFAULT FALSE,
    rule      CHAR(64)    NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS curr_rooms_uid_index ON curr_rooms (uid);
CREATE UNIQUE INDEX IF NOT EXISTS curr_rooms_id_index ON curr_rooms (id);

CREATE TABLE IF NOT EXISTS games
(
    id        CHAR(64)  NOT NULL UNIQUE,
    room_id   CHAR(64)  NOT NULL,
    room_name CHAR(64)  NOT NULL,
    users     INTEGER[] NOT NULL, -- 1~99 if robot
    is_rank   BOOLEAN   NOT NULL, -- type of game
    rule      CHAR(64)  NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS game_id_index ON games (id);
CREATE INDEX IF NOT EXISTS game_room_id_index ON games (room_id);

CREATE TABLE IF NOT EXISTS records
(
    game_id CHAR(64) NOT NULL,
    room_id CHAR(64) NOT NULL,
    number  INTEGER  NOT NULL, -- nth state in game
    state   JSON     NOT NULL,
    time    TIMESTAMP DEFAULT now()
);

CREATE INDEX IF NOT EXISTS record_game_id_index ON records (game_id);
CREATE INDEX IF NOT EXISTS record_room_id_index ON records (room_id);
CREATE INDEX IF NOT EXISTS record_time_index ON records (time);

CREATE TABLE IF NOT EXISTS ratings
(
    user_no INTEGER  NOT NULL,
    game_id CHAR(64) NOT NULL,
    diff    INTEGER  NOT NULL,
    rating  INTEGER  NOT NULL,
    time    TIMESTAMP DEFAULT now()
);

CREATE INDEX IF NOT EXISTS rating_user_id_index ON ratings (user_no);
CREATE INDEX IF NOT EXISTS rating_game_id_index ON ratings (game_id);
CREATE INDEX IF NOT EXISTS rating_time_index ON ratings (time);

CREATE TABLE IF NOT EXISTS rules
(
    rule_hash CHAR(64) NOT NULL UNIQUE,
    rule      JSON     NOT NULL,
    name      VARCHAR(255)
);

CREATE UNIQUE INDEX IF NOT EXISTS rules_hash_index ON rules (rule_hash);
CREATE INDEX IF NOT EXISTS rules_name_index ON rules (name);
