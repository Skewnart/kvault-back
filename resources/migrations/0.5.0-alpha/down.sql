ALTER TABLE users ADD last_name varchar(200);
UPDATE users SET last_name = '';
ALTER TABLE users ALTER COLUMN last_name SET NOT NULL;

ALTER TABLE users ADD first_name varchar(200);
UPDATE users SET first_name = '';
ALTER TABLE users ALTER COLUMN first_name SET NOT NULL;

ALTER TABLE users ADD email varchar(200);
UPDATE users SET email = '';
ALTER TABLE users ALTER COLUMN email SET NOT NULL;

ALTER TABLE users DROP COLUMN "type";

DROP TABLE invitations;