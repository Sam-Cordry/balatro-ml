CREATE TABLE consumables (
    "session" int4 NOT NULL,
    consumable consumable NOT NULL,
    negative bool DEFAULT false NOT NULL,
    CONSTRAINT consumables_sessions_fk FOREIGN KEY (
        "session"
    ) REFERENCES sessions ("session") ON DELETE CASCADE ON UPDATE CASCADE
);
