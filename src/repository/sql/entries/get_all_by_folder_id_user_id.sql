SELECT
    id,
    "name",
    description,
    is_favorite
FROM entries
WHERE folder_id = $1
  AND user_id = $2
ORDER BY name;