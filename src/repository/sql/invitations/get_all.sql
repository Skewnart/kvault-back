SELECT guid, created_at + duration "ends_at", is_active, users.username "invited_username"
FROM invitations
LEFT JOIN users ON invitations.user_id = users.id;