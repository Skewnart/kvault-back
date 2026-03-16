UPDATE folders
    SET enc_entries = $1
WHERE id = $2
    AND user_id = $3
RETURNING id;