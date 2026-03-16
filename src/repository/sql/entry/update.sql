UPDATE entries
SET enc_dats = $1
where id = $2
  and user_id = $3
RETURNING id;