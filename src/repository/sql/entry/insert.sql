INSERT INTO entries("name", description, password, is_favorite, folder_id, user_id)
VALUES (
           $1,
           $2,
           $3,
           $4,
           (SELECT id FROM folders  WHERE id = $5 AND user_id = $6),
           $6
       )
    RETURNING id;