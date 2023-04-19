use clap::{App, Arg};
use std::process::Command;
use tokio::runtime::Runtime;
use tokio::task::spawn_blocking;

fn main() {
    let matches = App::new("Kubectl Wrapper")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Wraps kubectl config")
        .arg(
            Arg::with_name("CONFIG_COMMAND")
                .help("Subcommand to be passed to kubectl config")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("ARGS")
                .help("Additional arguments for the kubectl config subcommand")
                .multiple(true)
                .last(true),
        )
        .get_matches();

    let config_command = matches.value_of("CONFIG_COMMAND").unwrap();
    let other_args: Vec<&str> = matches.values_of("ARGS").unwrap_or_default().collect();

    let mut command = Command::new("kubectl");
    command.arg("config").arg(config_command);
    command.args(&other_args);

    let output_result = Runtime::new()
        .unwrap()
        .block_on(async {
            spawn_blocking(move || command.output()).await.unwrap()
        });

    // Handle the Result type by using a match expression to check for success or error.
    // If there is an error, it will print the error message and exit with a non-zero status code.
    // If the command is successful, it will print the command output or error output depending on
    // the command's exit status.
    match output_result {
        Ok(output) => {
            if output.status.success() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            } else {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                std::process::exit(1);
            }
        }
        Err(error) => {
            eprintln!("Error: {:?}", error);
            std::process::exit(1);
        }
    }
}
