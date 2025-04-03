use tracing::info;

pub mod init;

fn main() -> eyre::Result<()> {
    init::init()?;
    info!("Hello, world!");
    Ok(())
}
