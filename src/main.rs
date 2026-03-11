use odoo_18_quotations_to_19::odoo_18::Odoo18JsonRPCClient;
use std::env::var;
use url::Url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    env_logger::init();

    log::info!("getting client...");
    let client_18 = Odoo18JsonRPCClient::new(
        Url::parse(var("ODOO_18_BASE_URL")?.as_str())?,
        var("ODOO_18_USER")?,
        var("ODOO_18_PASSWORD")?,
        var("ODOO_18_DATABASE")?,
    )
    .await?;
    log::info!("client got!");

    println!("uid = {:#?}", client_18.get_uid());
    println!("version = {:#?}", client_18.version().await?);
    Ok(())
}
