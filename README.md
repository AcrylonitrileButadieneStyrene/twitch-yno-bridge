# twitch-yno-bridge
Link twitch chat to a bot playing on YNO.

# Setup
Change which stream's chat the controller is hardcoded to listen to, then build the controller and script.

In a directory, put the `controller` binary, `script.js`, and a new file named `trusted_users.txt`.

Put 1 twitch username per line starting with a hashtag in trusted_users.txt for admins.

In a browser, using a userscript manager and a userstyle manager, install `script.user.js` and `style.user.css`.

Run the `controller` binary and open YNO. If you are using Firefox in kiosk mode then open `http://localhost:6248/redirect` instead. It has a few seconds of delay before redirecting to help with the userscript manager not initializing in time.

# Building
- `cargo build --release`
- `rollup -c`
