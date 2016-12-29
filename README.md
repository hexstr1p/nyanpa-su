# nyanpa-su
A url shortener.

One implementation is written in node. The other is written in Rust.

---
With rust, openssl has some problems so it has to be build with
`OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include DEP_OPENSSL_INCLUDE=/usr/local/opt/openssl/include cargo build
`
