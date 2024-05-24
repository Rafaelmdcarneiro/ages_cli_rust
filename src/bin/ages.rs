use ages_cli::Command;

fn main() {
    // TODO: Get the command via command-line argument parsing
    let command = Command::Hello;

    match command {
        Command::Hello => hello()
    }
}

// TODO: Move each command to a separate module and exposed in `lib.rs`
fn hello() {
    println!("Hello from Ages!")
}
