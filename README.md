# kubectl_wrapper
A command-line application in Rust that wraps kubectl config,
you'll first need to install Rust and Cargo if you haven't already. 
You can find instructions on how to do this on the official Rust website: 

https://www.rust-lang.org/tools/install.

Once Rust and Cargo are installed, follow these steps:

Build and Run the project

$ cargo build
$ cargo run -- <subcommand> [<args>] 

Replace <subcommand> with a kubectl config subcommand like use-context or get-contexts, and provide additional arguments as needed.

For example:

$ cargo run -- use-context my-cluster
