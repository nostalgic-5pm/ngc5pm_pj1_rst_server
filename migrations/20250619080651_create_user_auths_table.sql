-- Add migration script here
CREATE TABLE IF NOT EXISTS user_auths (
    user_id BIGINT NOT NULL,
    current_hashed_password VARCHAR(128) NOT NULL,
    prev_hashed_password_1 VARCHAR(128),
    prev_hashed_password_2 VARCHAR(128),
    login_fail_times SMALLINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id),
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);
