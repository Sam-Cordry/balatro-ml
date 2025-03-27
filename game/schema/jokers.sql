CREATE TABLE jokers (
    "session" int4 NOT NULL,
    joker "joker" NOT NULL,
    edition "joker_edition" NOT NULL,
    "index" int4 NOT NULL,
    mult int4 NULL,
    xmult float4 NULL,
    chips int4 NULL,
    "rank" "card_rank" NULL,
    suit "card_suit" NULL,
    "money" int4 NULL,
    CONSTRAINT jokers_sessions_fk FOREIGN KEY ("session") REFERENCES public.sessions ("session") ON DELETE CASCADE ON UPDATE CASCADE
);
