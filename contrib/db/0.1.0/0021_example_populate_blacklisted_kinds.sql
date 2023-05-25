-- RUN THIS SCRIPT LIKE THIS: psql postgres -U spamvoid -f 0021_populate_blacklisted_kinds.sql

-- Blocks NIP-95 file storage
INSERT INTO blacklisted_kinds (kind) VALUES (1064);
-- IYKYK ;)
INSERT INTO blacklisted_kinds (kind) VALUES (70202);
