SELECT guid
FROM invitations
WHERE guid = $1
  AND is_active = TRUE
  AND created_at + duration >= now()
  AND user_id IS NULL;