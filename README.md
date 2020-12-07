# tls_echo
## Simple TLS echo server made in Rust with Tokio and OpenSSL
### For testing mTLS or using as boilerplate
#### Instructions:
Run the server with `cargo run`
<br/>
In another shell, execute something like
```
openssl s_client -key client.key -cert client.crt -CAfile ca.crt -connect localhost:8443
```
and start typing whatever you want
##### Note: You must have the server.key and certs-chain.crt (server.crt + ca.crt) files present in ssl/
