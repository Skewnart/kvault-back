INSERT INTO entries(enc_datas, user_id)
VALUES (
           $1,
           $2
       )
    RETURNING id;