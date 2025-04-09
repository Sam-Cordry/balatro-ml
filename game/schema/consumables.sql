CREATE TABLE public.consumables (
    "session" int4 NOT NULL,
    "consumable" public."consumable" NOT NULL,
    negative bool DEFAULT false NOT NULL,
    CONSTRAINT consumables_sessions_fk FOREIGN KEY (
        "session"
    ) REFERENCES public.sessions ("session") ON DELETE CASCADE ON UPDATE CASCADE
);
