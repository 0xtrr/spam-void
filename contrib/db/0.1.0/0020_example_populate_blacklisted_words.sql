-- RUN THIS SCRIPT LIKE THIS: psql spamvoid -U spamvoid -h localhost -f 0020_populate_blacklisted_words.sql

-- Chinese spam example
INSERT INTO blacklisted_words (word) VALUES ('http://user.damus.place');