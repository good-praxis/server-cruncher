# Server Cruncher

[![dependency status](https://deps.rs/repo/github/good-praxis/server-cruncher/status.svg)](https://deps.rs/repo/github/good-praxis/server-cruncher)
[![Build Status](https://github.com/good-praxis/server-cruncher/workflows/CI/badge.svg)](https://github.com/good-praxis/server-cruncher/actions?workflow=CI)

## Status

Currently in ongoing development towards basic features.

- [] Fetch all servers
- [] Fetch all images
- [] Combine servers and images into identities
- [] UI configurable API endpoint
- [] Shut down remote servers
- [] Create backup from remote server
- [] Destroy remote server, tracking the backup
- [] Provision a new server on basis of backup

After initial release, planned features are:

- [] Secure shutdown of applications on servers before shutting them down
- [] Configuration based on provisioned server data
- [] Cost overview

### Testing locally

To test with mocking enabled, run
`env RUSTFLAGS='--cfg mock' cargo test`
