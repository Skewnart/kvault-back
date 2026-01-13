UPDATE entries
SET "name" = $1,
    description = $2,
    password = $3,
    is_favorite = $4
where id = $5
  and user_id = $6
RETURNING id;