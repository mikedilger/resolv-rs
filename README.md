# Resolv

DNS resolution via glibc.

This uses `libresolv.so` which is typically configured via `/etc/resolv.conf` to do DNS
resolution.  It allows you to look up DNS resource records of any type (e.g. A, AAAA, MX, TXT,
etc), use recursion (if your system's DNS resolver permits it), and perform the DNS search
algorithm to complete incomplete names and use your `/etc/hosts` file.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
