# tls_echo
## Simple TLS echo server made in Rust with Tokio and OpenSSL crates
### Just for testing
#### Instructions:
Run the server with `cargo run`
<br/>
In another shell, execute something like
```
openssl s_client -key client-key.pem -cert client-cert.pem -CAfile CA-bundle.pem -connect localhost:8443
```
and start typing whatever you want
##### Note: You must have the server and client certificates, as well as use an available port for the server
