use sacp::ConnectTo;
use sacp_test::testy::Testy;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();
    Testy::new().connect_to(sacp_tokio::Stdio::new()).await?;
    Ok(())
}
