-- This file should undo anything in `up.sql`

DROP TABLE User;
DROP FUNCTION IF EXISTS get_pw_hash;
