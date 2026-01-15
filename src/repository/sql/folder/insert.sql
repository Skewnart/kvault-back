INSERT INTO folders(name, user_id)
VALUES ($1, $2)
RETURNING id;
