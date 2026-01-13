UPDATE entries
SET folder_id = $1
WHERE id = $2
  AND user_id = $3;