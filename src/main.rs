mod config;
mod paybyphone;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::read("config.yaml")?;
    for account in config.accounts {
        let mut pay_by_phone = paybyphone::PayByPhone::new(
            account.plate,
            account.lot,
            account.rate,
            account.pay_by_phone.login,
            account.pay_by_phone.password,
            account.pay_by_phone.payment_account_id,
        );
        pay_by_phone.init().await?;
        println!("{}", pay_by_phone.get_vehicles().await)
    }
    Ok(())
}
