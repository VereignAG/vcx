CREATE TABLE IF NOT EXISTS devices (
    seq_num SERIAL PRIMARY KEY,
    account_id BINARY(16) NOT NULL UNIQUE,
    token VARCHAR(256) DEFAULT NULL,
    platform VARCHAR(16) DEFAULT NULL,
    FOREIGN KEY (account_id) REFERENCES accounts(account_id)
    ON DELETE CASCADE
);
