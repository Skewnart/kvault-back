DELETE FROM entries
WHERE id = ANY($1)
  AND user_id = $2
RETURNING id;