SELECT envelope
FROM users
WHERE id = $1
  AND envelope IS NOT NULL;