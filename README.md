## Known issue
The websocket service may be broken because of the missing `sec-websocket-protocol` header in handshake response. You may add it by modifying the `ntex` crate's code. **The release binary already fix that**.

## Usage
Add these to your hosts.
```
127.0.0.1 hjm.rebe.capcom.com
127.0.0.1 mtm.rebe.capcom.com
127.0.0.1 40912.playfabapi.com
```
Trust the certificate at ![static/obt-wilds.crt](static/obt-wilds.crt) (or you can create one by yourself).

Start this program working at ![static](./static/), also specifying the certificate ![crt](./static/obt-wilds.crt) and ![key](./static/obt-wilds.key) pem file. You can check the `--help` option.

Open the game and create private lobby or play solo.
