CREATE TABLE folders (
    id bigserial NOT NULL,
    "name" text NOT NULL,
    user_id bigserial NOT NULL,
    CONSTRAINT folders_pk PRIMARY KEY (id),
    CONSTRAINT folders_unique UNIQUE ("name",user_id),
    CONSTRAINT folders_users_fk FOREIGN KEY (user_id) REFERENCES users(id)
);
