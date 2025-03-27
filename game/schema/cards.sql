CREATE TABLE cards (
    "session" int4 NOT NULL,
    "rank" "card_rank" NOT NULL,
    suit "card_suit" NOT NULL,
    chips int4 NOT NULL,
    enhancement card_enhancement NULL,
    edition card_edition NOT NULL,
    seal card_seal NULL,
    CONSTRAINT cards_sessions_fk FOREIGN KEY ("session") REFERENCES sessions ("session") ON DELETE CASCADE ON UPDATE CASCADE
);
