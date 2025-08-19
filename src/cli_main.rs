mod cli;

fn main() {
    env_logger::init();

    if let Err(e) = cli::run_cli() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
