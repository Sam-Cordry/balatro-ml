CREATE TABLE public.jokers (
	"session" int4 NOT NULL,
	"joker" public."joker" NOT NULL,
	edition public."joker_edition" DEFAULT 'base'::joker_edition NOT NULL,
	"index" int4 NOT NULL,
	mult int4 NULL,
	xmult float4 NULL,
	chips int4 NULL,
	"rank" public."card_rank" NULL,
	suit public."card_suit" NULL,
	"money" int4 NULL,
	CONSTRAINT jokers_sessions_fk FOREIGN KEY ("session") REFERENCES public.sessions("session") ON DELETE CASCADE ON UPDATE CASCADE
);
