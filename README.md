# Battlesnake

Just having a play with Battlesnake (see (https://play.battlesnake.com/)[https://play.battlesnake.com/]).

# Deployment

Manual at the moment. This assumes deployment to a linux machine. Here mainly to trigger my memory!

These steps assume you'll be running it as `root`, which is strongly discouraged!

First, build using `cargo cross` (I'm often on a Mac):

```sh
cargo install cross

cross build --release --target x86_64-unknown-linux-musl
```

Then, copy binary onto the deployment machine.

```sh
scp ./target/x86_64-unknown-linux-musl/release/battlesnake root@some.machine.address:battlesnake/battlesnake
```

Tell the system about our new service by putting something like this in `/etc/systemd/system/battlesnake.service`:

```
[Unit]
Description=Battlesnake Service
After=network.target

[Service]
Type=simple
WorkingDirectory=/root/battlesnake
ExecStart=/root/battlesnake/battlesnake -l 0.0.0.0:8888
User=root
Group=root
Restart=always

[Install]
WantedBy=multi-user.target
```

Once the above config is in place, we can do things like:

```sh
# Enable the service starting automatically:
systemctl enable battlesnake.service
# Stop the service from starting automatically:
systemctl disable battlesnake.service
# We can start and stop the service in the current session:
systemctl {start|stop|..} battlesnake.service
# We can edit the service file (or `nano /etc/systemd/system/battlesnake.service`):
systemctl edit --full battlesnake.service
# We can view all logs:
journalctl -u battlesnake.service
```