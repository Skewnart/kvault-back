SELECT id,
       name
FROM folders
WHERE user_id = $1
ORDER BY name;