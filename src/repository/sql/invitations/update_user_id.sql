UPDATE invitations
SET user_id = $1
WHERE guid = $2
    AND is_active = TRUE
    AND created_at + duration >= now()
    AND user_id IS NULL
RETURNING guid;