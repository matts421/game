use game::networking::echo_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    echo_server().await
}
