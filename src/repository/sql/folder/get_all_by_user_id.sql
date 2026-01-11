SELECT id,
       name
FROM folders
WHERE user_id = $1;
