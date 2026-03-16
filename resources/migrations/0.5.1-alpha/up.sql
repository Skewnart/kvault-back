ALTER TABLE users ADD envelope jsonb;

ALTER TABLE users ADD enc_folders text;
ALTER TABLE folders ADD enc_entries text;

ALTER TABLE entries RENAME COLUMN "password" TO enc_datas;

ALTER TABLE folders DROP CONSTRAINT folders_unique;
ALTER TABLE folders DROP COLUMN "name";
ALTER TABLE entries DROP COLUMN is_favorite;
ALTER TABLE entries DROP COLUMN "name";
ALTER TABLE entries DROP COLUMN description;
ALTER TABLE entries ALTER COLUMN user_id DROP DEFAULT;
ALTER TABLE folders ALTER COLUMN user_id DROP DEFAULT;
