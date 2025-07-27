CREATE TABLE cards (
    "session" int4 NOT NULL,
    "rank" card_rank NOT NULL,
    suit card_suit NOT NULL,
    chips int4 NOT NULL,
    enhancement card_enhancement NULL,
    edition card_edition DEFAULT 'base'::card_edition NOT NULL,
    seal card_seal NULL,
    in_deck bool DEFAULT true NOT NULL,
    in_hand bool DEFAULT false NOT NULL,
    "index" int4 NOT NULL,
    CONSTRAINT cards_sessions_fk FOREIGN KEY (
        "session"
    ) REFERENCES sessions ("session") ON DELETE CASCADE ON UPDATE CASCADE
);
