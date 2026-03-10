ALTER TABLE users DROP COLUMN "first_name";
ALTER TABLE users DROP COLUMN "last_name";
ALTER TABLE users DROP COLUMN "email";

ALTER TABLE users ADD "type" text DEFAULT 'USER' NOT NULL;

CREATE TABLE invitations (
    guid uuid DEFAULT gen_random_uuid() NOT NULL,
    created_at timestamp DEFAULT now() NOT NULL,
    duration interval NOT NULL,
    is_active boolean DEFAULT TRUE NOT NULL,
    user_id bigint NULL,
    CONSTRAINT invitations_pk PRIMARY KEY (guid),
    CONSTRAINT invitations_users_fk FOREIGN KEY (user_id) REFERENCES users(id)
);
