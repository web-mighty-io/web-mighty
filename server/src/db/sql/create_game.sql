CREATE TABLE game
(
    id      UUID      NOT NULL,
    room_id UUID      NOT NULL,
    users   INTEGER[] NOT NULL,
    start   TIMESTAMP NOT NULL,
    finish  TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX game_id_index ON game (id);
CREATE INDEX game_room_id_index ON game (room_id);

CREATE TABLE record
(
    game_id    UUID      NOT NULL,
    room_id    UUID      NOT NULL,
    number     INTEGER   NOT NULL, -- nth state in game
    diff_state JSON      NOT NULL, -- difference of each state
    time       TIMESTAMP NOT NULL
);

CREATE INDEX record_game_id_index ON record (game_id);
CREATE INDEX record_room_id_index ON record (room_id);
