UPDATE users SET enc_folders = '{}';
UPDATE folders SET enc_entries = '{}';
UPDATE entries SET enc_datas = '{}';

ALTER TABLE users ALTER COLUMN enc_folders TYPE jsonb USING enc_folders::jsonb;
ALTER TABLE folders ALTER COLUMN enc_entries TYPE jsonb USING enc_entries::jsonb;
ALTER TABLE entries ALTER COLUMN enc_datas TYPE jsonb USING enc_datas::jsonb;
