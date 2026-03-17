use std::process;

#[tokio::main]
async fn main() {
    env_logger::init();
    if let Err(err) = odoo_18_quotations_to_19::run().await {
        eprintln!("{err}");
        eprintln!("{}", err.backtrace());
        process::exit(1);
    }
}
