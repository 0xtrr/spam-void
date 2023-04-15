# SpamVoid

## Description
Let the spammers pay


## DB

### Create user

1. Log into your DB, e.g. with `sudo -u postgres psql`
2. Run `CREATE USER spamvoid WITH NOSUPERUSER NOCREATEROLE CREATEDB INHERIT LOGIN PASSWORD 'spamvoid';`
3. Quit the db console `\q`
4. Verify that the user works: `psql -U spamvoid -d spamvoid -h localhost`

CREATE USER nostr WITH NOSUPERUSER NOCREATEROLE CREATEDB INHERIT LOGIN PASSWORD 'nostr';