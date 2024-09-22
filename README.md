# rncat

Simple reverse shell to learn Rust

## Usage

rncat \<COMMAND\> [options]

### Commands

- `server`: Run this on local machine (will send and receive the commands and its output, respectively)
- `client`: Run this on remote machine where the sent commands from server will be executed on
- `help`: Print this message or the help of the given subcommand(s)

### Options

-a, --address \<ADDRESS\>

-p, --port \<PORT\>

-h, --help

## Demo



https://github.com/user-attachments/assets/f41aaa66-f1b2-41d9-9a44-7960909f5a61


TODO:
- [ ] Send separate commands for each client connection
