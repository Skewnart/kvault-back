ALTER TABLE users ALTER COLUMN enc_folders TYPE text USING enc_folders::text;
ALTER TABLE folders ALTER COLUMN enc_entries TYPE text USING enc_entries::text;
ALTER TABLE entries ALTER COLUMN enc_datas TYPE text USING enc_datas::text;
