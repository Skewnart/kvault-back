SELECT first_name,
       last_name,
       username
FROM users
WHERE id = $1;