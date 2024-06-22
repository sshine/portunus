//! # Portunus - SSH AuthorizedKeys Manager
//!
//! ![CI](https://github.com/sshine/portunus/actions/workflows/main.yml/badge.svg)
//!
//! Distribute your SSH AuthorizedKeys for multiple users with multiple keys to multiple machines.
//!
//! Portunus supports importing its configuration from a git repository, and user SSH keys from GitHub.
//!  
//! ## How does it work?
//!  
//!  - `portunus-authkeys-cmd` is called by `sshd` using the `AuthorizedKeysCommand` setting.
//!  - `portunus-authkeys-cmd` asks `portunus-keycached` if keys are authorized via UNIX or TCP socket.
//!  - `portunus-keycached` gives a cached answer and syncs with its sources or other instances.
//!  
//! You install `portunus-authkeys-cmd` and `portunus-keycached` on each system under management.
//!  
//! You may install another `portunus-keycached` on one or more systems reachable via TCP.
//!  
//! You define which users have which keys and which users have access to which servers.
//!  
//! ## How do I install Portunus?
//!  
//! ### Debian
//!
//! ### NixOS
//!
//! ### Cargo
//!
//! ## Example deployments
//!
//! ### Single server
//!
//! ### Multi server

mod keybase;
