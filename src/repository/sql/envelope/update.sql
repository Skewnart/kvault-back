UPDATE users
SET envelope = $1
WHERE id = $2
  AND envelope IS NULL
RETURNING id;