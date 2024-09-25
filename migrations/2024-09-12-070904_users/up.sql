-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Users
CREATE TABLE IF NOT EXISTS Users (
    id UUID,
    name TEXT,
    email TEXT NOT NULL,
    password TEXT,
    oauth_provider TEXT NOT NULL,
    oauth_user_id TEXT NOT NULL,
    access_token TEXT NOT NULL,
    refresh_token TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(Id)
);

-- Function for generating password hashes
CREATE OR REPLACE FUNCTION get_pw_hash(password TEXT)
RETURNS TEXT LANGUAGE plpgsql AS $$
DECLARE
    encrypted_pw TEXT;
BEGIN
    SELECT INTO encrypted_pw crypt(password, gen_salt('md5'));
    RETURN encrypted_pw;
END; 
$$;

-- Function for checking password hashes
CREATE OR REPLACE FUNCTION check_pw_hash(password TEXT, hash TEXT)
RETURNS TEXT LANGUAGE plpgsql AS $$
DECLARE
    generated_hash TEXT;
BEGIN
    SELECT INTO generated_hash crypt(password, hash);
    IF generated_hash = hash THEN
        return true;
    END IF;
    RETURN false;
END; 
$$;

-- Function for checking user passwords
CREATE OR REPLACE FUNCTION check_pw(user_email TEXT, user_password TEXT)
RETURNS TEXT LANGUAGE plpgsql AS $$
DECLARE
    pw_hash TEXT;
BEGIN
    SELECT password INTO pw_hash from Users where email = user_email;
    IF check_pw_hash(user_password, pw_hash) THEN
        return true;
    END IF;
    RETURN false;
END; 
$$;

