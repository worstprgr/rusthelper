use clap::Parser;

// Source: https://rust-cli.github.io/book/tutorial/cli-args.html
#[derive(Parser)]
struct Cli {
    testarg: String,
    testint: i32,
}

fn main() {
    let args = Cli::parse();
    println!("Testarg: {}\nTestint: {}", args.testarg, args.testint)
    // run with cargo run -- hello 123
}
