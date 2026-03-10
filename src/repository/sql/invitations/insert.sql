INSERT INTO invitations (duration)
VALUES (($1||' '|| $2)::interval)
RETURNING guid;