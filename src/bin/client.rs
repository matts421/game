use game::networking::echo_client;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    echo_client().await
}
