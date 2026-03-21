SELECT
    enc_datas
FROM entries
WHERE id = $1
  AND user_id = $2;