DROP TABLE IF EXISTS curr_rooms;

CREATE TABLE curr_rooms
(
    uuid       UUID      NOT NULL,
    id         INTEGER   NOT NULL,
    users      INTEGER[] NOT NULL,
    is_gaming  BOOLEAN   NOT NULL DEFAULT FALSE,
    rule       JSON      NOT NULL
);

DROP TABLE IF EXISTS games;

CREATE TABLE games
(
    id      UUID      NOT NULL,
    room_id UUID      NOT NULL,
    users   INTEGER[] NOT NULL, -- 0 if robot
    is_rank BOOLEAN   NOT NULL, -- type of game
    rule    JSON      NOT NULL
);

CREATE UNIQUE INDEX game_id_index ON games (id);
CREATE INDEX game_room_id_index ON games (room_id);

DROP TABLE IF EXISTS record;

CREATE TABLE record
(
    game_id UUID    NOT NULL,
    room_id UUID    NOT NULL,
    number  INTEGER NOT NULL, -- nth state in game
    state   JSON    NOT NULL,
    time    TIMESTAMP DEFAULT now()
);

CREATE INDEX record_game_id_index ON record (game_id);
CREATE INDEX record_room_id_index ON record (room_id);

DROP TABLE IF EXISTS rating;

CREATE TABLE rating
(
    user_no INTEGER NOT NULL,
    game_id UUID    NOT NULL,
    diff    INTEGER NOT NULL,
    rating  INTEGER NOT NULL,
    time    TIMESTAMP DEFAULT now()
);

CREATE INDEX rating_user_id_index ON rating (user_no);
CREATE INDEX rating_game_id_index ON rating (game_id);
