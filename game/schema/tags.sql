CREATE TABLE tags (
    "session" int4 NOT NULL,
    tag public.tag NOT NULL,
    "index" int4 NOT NULL,
    CONSTRAINT tags_session_fk FOREIGN KEY (
        "session"
    ) REFERENCES public.sessions ("session") ON DELETE CASCADE ON UPDATE CASCADE
);
