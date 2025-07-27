CREATE TABLE tags (
    "session" int4 NOT NULL,
    tag tag NOT NULL,
    "index" int4 NOT NULL,
    CONSTRAINT tags_session_fk FOREIGN KEY (
        "session"
    ) REFERENCES sessions ("session") ON DELETE CASCADE ON UPDATE CASCADE
);
