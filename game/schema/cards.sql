CREATE TABLE public.cards (
    "session" int4 NOT NULL,
    "rank" public."card_rank" NOT NULL,
    suit public."card_suit" NOT NULL,
    chips int4 NOT NULL,
    enhancement public."card_enhancement" NULL,
    edition public."card_edition" DEFAULT 'base'::card_edition NOT NULL,
    seal public."card_seal" NULL,
    in_deck bool DEFAULT true NOT NULL,
    in_hand bool DEFAULT false NOT NULL,
    "index" int4 NOT NULL,
    CONSTRAINT cards_sessions_fk FOREIGN KEY (
        "session"
    ) REFERENCES public.sessions ("session") ON DELETE CASCADE ON UPDATE CASCADE
);
