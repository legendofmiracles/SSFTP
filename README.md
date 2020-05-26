[![Build Status](https://travis-ci.com/AVCADO/SSFTP.svg?branch=master)](https://travis-ci.com/AVCADO/SSFTP)
## SSFTP
A simple, lightweight, flexible FTP and SFTP client.

## Behind the acronym
**S**imple
**S**ecure
**F**ile
**T**ransfer
**P**rotocol


## Why?
Have you ever had that time, where you forgot a file on
your desktop, and you want to have it on your laptop,
but you don't have a USB? SSFTP saves the day.


(it's basically OneDrive, CLI edition, not made by Microsoft,
and open source!)


## Technologies used:

- Rust

- Rust openssl crate

- Rust ftp crate


All credit goes to their authors.


## Slack

[Join the slack channel](https://join.slack.com/t/ssftpdevelopment/shared_invite/zt-ebx867f9-OJvLwaTESM28Abe5EJyzWA)


### Usage

## Ways to execute

# Building from source

If you would like to build from source,
please write the following,
it is OS-sensitive


WINDOWS

POWERSHELL IS ONLY SUPPORTED.

```ps
wget https://win.rustup.rs/x86_64 
./rustup-init.exe
```
Run the installer.
then,
run this:
```ps
git clone https://github.com/AVCADO/SSFTP
cd SSFTP
cargo run
```

UNIX
``` 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/AVCADO/SSFTP
cd SSFTP
cargo run
```