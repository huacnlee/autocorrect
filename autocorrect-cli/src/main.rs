#[tokio::main]
async fn main() {
    autocorrect_cli::run(std::env::args_os()).await;
}
