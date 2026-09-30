UPDATE users
SET password_attempts = 0
WHERE id = $1
RETURNING password_attempts;