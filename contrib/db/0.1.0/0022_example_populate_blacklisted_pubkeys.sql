-- RUN THIS SCRIPT LIKE THIS: psql postgres -U spamvoid -f 0021_populate_blacklisted_pubkeys.sql

-- Random deleted account example
INSERT INTO blacklisted_pubkeys (pubkey) VALUES ("ba562fbd114f6243e71a91c15130eb3a3d999d9c8b56060d9f56255718cb2932");
