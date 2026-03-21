INSERT INTO users(username, password, envelope, enc_folders)
VALUES ($1, $2, $3, $4)
RETURNING id;
