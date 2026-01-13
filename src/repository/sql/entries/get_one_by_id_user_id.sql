SELECT
    "name",
    description,
    is_favorite
FROM entries
WHERE id = $1
  AND user_id = $2;