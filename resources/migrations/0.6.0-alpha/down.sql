ALTER TABLE users DROP COLUMN envelope;

ALTER TABLE users DROP COLUMN enc_folders;
ALTER TABLE folders DROP COLUMN enc_entries;

ALTER TABLE entries RENAME COLUMN enc_datas TO "password";

ALTER TABLE folders ADD "name" text;
ALTER TABLE folders ADD CONSTRAINT folders_unique UNIQUE ("name",user_id);
ALTER TABLE entries ADD is_favorite boolean DEFAULT false NOT NULL;
ALTER TABLE entries ADD "name" text;
ALTER TABLE entries ADD description text;
