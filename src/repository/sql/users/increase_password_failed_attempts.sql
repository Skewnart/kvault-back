UPDATE users
SET password_attempts = (SELECT password_attempts + 1 FROM users WHERE id = $1)
WHERE id = $1
RETURNING password_attempts;