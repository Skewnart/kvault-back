UPDATE entries
SET "name" = $1,
    description = $2,
    password = $3,
    is_favoris = $4
where id = $5
  and user_id = $6;