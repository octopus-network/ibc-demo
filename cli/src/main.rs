mod command;
mod error;
mod ibc_logic;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = tokio::spawn(async move {
        let _ = command::run().await;
    });

    let _ = tokio::join!(result);

    Ok(())
}
