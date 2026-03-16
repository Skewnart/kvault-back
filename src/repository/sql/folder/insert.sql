INSERT INTO folders(user_id)
VALUES ($1)
RETURNING id;
