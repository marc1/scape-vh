use crossterm::Result;

#[tokio::main]
async fn main() -> Result<()> {
    scape_vh::init().await?;

    Ok(())
}
