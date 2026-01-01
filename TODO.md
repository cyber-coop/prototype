# TODO

- [ ] Create an option to not register in database
- [x] Create a thread that would queue data to save in the database
- [x] ~~Prometheus + grafana~~ influxdb data collector and syncing monitoring
- [x] Dockerfile
- [x] Remove warnings
- [ ] Improve development process 
    - [ ] Use `init.sh` to create tables
- [ ] Improve sql query (in NOTES.md) to avoid the encoding call
- [ ] Properly manage forks!
- [ ] Rename `networks.rs` to `blockchains.rs`
- [ ] Rename `peer.rs` to `connection.rs`
- [ ] Support IPv6
- [x] Need transaction version!!
- [x] Resolve the mistery of the missing txid `8f311e28fb852de8456b1a55e68cf8e9be501fbfd8e386cc9c551b41f3e0809b` (probably wrong hash...) ... hash registered `\x83c4938e238bef8bf2b64333abf106226c68abd36a80f5b916cf1b6d8fe8d5bc`
- [ ] Adding connection timeout
- [ ] Need nodes fallback (or we can use dns look up to find nodes)
- [ ] ~~Send an email once a long sql script is done (see `mail`)~~
- [ ] Create table on database at start