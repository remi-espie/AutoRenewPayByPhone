mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{:?}", config::read("config.yaml"));
    Ok(())
}
