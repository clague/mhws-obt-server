## Known issue
The websocket service may be broken because of the missing `sec-websocket-protocol` header in handshake. You may add it by modifying the `ntex` crate's code.
