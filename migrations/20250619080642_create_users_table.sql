-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    user_id BIGSERIAL,
    public_id VARCHAR(21) NOT NULL UNIQUE,
    randomart TEXT NOT NULL,
    user_name VARCHAR(64) NOT NULL UNIQUE,
    first_name VARCHAR(64),
    last_name VARCHAR(64),
    email VARCHAR(254) UNIQUE,
    phone VARCHAR(16) UNIQUE,
    birth_date DATE,
    status SMALLINT NOT NULL DEFAULT 0,
    role SMALLINT NOT NULL DEFAULT 0,
    last_login_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id)
);
