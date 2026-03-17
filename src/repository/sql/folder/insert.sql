INSERT INTO folders(user_id, enc_entries)
VALUES ($1, $2)
RETURNING id;
