CREATE TABLE folders (
    id bigserial NOT NULL,
    "name" text NOT NULL,
    user_id bigserial NOT NULL,
    CONSTRAINT folders_pk PRIMARY KEY (id),
    CONSTRAINT folders_unique UNIQUE ("name",user_id),
    CONSTRAINT folders_users_fk FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE entries (
    id bigserial NOT NULL,
    "name" text NOT NULL,
    description text NULL,
    "password" text NOT NULL,
    is_favoris boolean DEFAULT false NOT NULL,
    folder_id bigserial NOT NULL,
    user_id bigserial NOT NULL,
    CONSTRAINT entries_pk PRIMARY KEY (id),
    CONSTRAINT entries_folders_fk FOREIGN KEY (folder_id) REFERENCES folders(id),
    CONSTRAINT entries_users_fk FOREIGN KEY (user_id) REFERENCES users(id)
);
