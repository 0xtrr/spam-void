-- RUN THIS SCRIPT LIKE THIS: psql postgres -U spamvoid -h localhost -f 0002_create_database.sql

-- Create the 'spamvoid' database
CREATE DATABASE spamvoid;

-- Grant all priviliges to your user
GRANT ALL PRIVILEGES ON DATABASE spamvoid TO spamvoid;

-- Switch to the 'spamvoid' database
\c spamvoid;

-- Create the 'blacklisted_words' table
CREATE TABLE blacklisted_words (
    id SERIAL PRIMARY KEY,
    word TEXT NOT NULL
);

-- Create the 'categories' table
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

-- Create the 'blacklisted_words_categories' table to establish a many-to-many relationship
CREATE TABLE blacklisted_words_categories (
    blacklisted_word_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    PRIMARY KEY (blacklisted_word_id, category_id),
    FOREIGN KEY (blacklisted_word_id) REFERENCES blacklisted_words (id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories (id) ON DELETE CASCADE
);

-- Create the 'blacklisted_pubkeys' table
CREATE TABLE blacklisted_pubkeys (
    id SERIAL PRIMARY KEY,
    pubkey TEXT NOT NULL
);

-- Create the 'blacklisted_kinds' table
CREATE TABLE blacklisted_kinds (
    id SERIAL PRIMARY KEY,
    kind INTEGER NOT NULL
);

-- Create an index on the 'word' column for efficient queries
CREATE INDEX idx_blacklisted_words_word ON blacklisted_words(word);
-- Create an index on the 'pubkey' column for efficient queries
CREATE INDEX idx_blacklisted_pubkeys_pubkey ON blacklisted_pubkeys(pubkey);
-- Create an index on the 'kinds' column for efficient queries
CREATE INDEX idx_blacklisted_kinds_kind ON blacklisted_kinds(kind);