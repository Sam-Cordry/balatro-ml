CREATE TABLE consumables (
    "session" int4 NOT NULL,
    "consumable" "consumable" NOT NULL,
    negative bool NOT NULL,
    CONSTRAINT consumables_sessions_fk FOREIGN KEY ("session") REFERENCES public.sessions("session") ON DELETE CASCADE ON UPDATE CASCADE
);
