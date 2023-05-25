# Setup

This document shows how to set up the spamvoid relay filter.

This guide assumes that you have set up a basic postgresql database already. There are many good guides for doing this online so just Google it or ask ChatGPT for help.

## Set up the spamvoid database

These commands are based on you being in the root level of the project folder. All the mentioned database scripts can be found under the contrib/db/ folder.

1. Create a new user by running the `0001_create_user.sql` script.
2. Create the database and set up the necessary tables by running the `0002_create_database.sql` script.

Now the database is set up but we don't block anything yet. To blacklist kinds, pubkeys and specific words, take a look at the example scripts provided in the contrib/db/ folder. As it is now, you have to manually add every word, kind or pubkey you want to blacklist.

## Install the service as a Linux service

1. Create a configuration folder and place the example configuration in the new folder
```bash
# Create the folder
sudo mkdir /etc/spamvoid
# Give ownership to the current user
sudo chown $USER:$USER /etc/spamvoid
# Copy in the configuration file
cp example-config.toml /etc/spamvoid/config.toml
```

2. Edit the configuration to fit your own setup

3. Build the SpamVoid binary from source and copy it to the correct folder. This step requires you to have Cargo installed on your machine.
```bash
# Build the binary
cargo build --release
# Copy the binary to the correct folder
sudo cp target/release/spam_void /usr/local/bin/
```

4. Set up logging folder (this is just an irritating temporary solution, will fix this later so admins doesn't have to think about it)
```bash
# Create the folder the log should be written to
sudo mkdir /var/log/spamvoid
# Ensure correct ownership so that spamvoid is able to write the log
sudo chown $USER:$USER /var/log/spamvoid
```

5. Put the spamvoid.service file in the /etc/systemd/system/ folder and call it spamvoid.service.
```bash
# Place the service file in the correct folder
# NOTE: Remember to change the username in this file before moving on to the next step
sudo cp contrib/spamvoid.service /etc/systemd/system/spamvoid.service
# Reload the system daemon to load the new service
sudo systemctl daemon-reload
# Enable the service on startup
sudo systemctl enable spamvoid.service
# Start the service
sudo systemctl start spamvoid.service
# Check if the service is running (should be green and say Active)
sudo systemctl status spamvoid.service
```
