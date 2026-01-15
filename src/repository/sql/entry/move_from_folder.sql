UPDATE entries
SET folder_id =
(
    SELECT id
    FROM folders
    WHERE id = $1
    AND user_id = $3
)
WHERE id = $2
  AND user_id = $3
RETURNING id;