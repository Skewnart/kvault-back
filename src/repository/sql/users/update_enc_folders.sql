UPDATE users
    SET enc_folders = $1
WHERE id = $2
RETURNING id;