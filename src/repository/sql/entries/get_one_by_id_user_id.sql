SELECT
    "name",
    description,
    is_favoris
FROM entries
WHERE id = $1
  AND user_id = $2;