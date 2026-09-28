use colored::Colorize;
use std::process;

#[tokio::main]
async fn main() {
    env_logger::init();
    let run_res = odoo_json2_toolbox::run().await;
    if let Err(err) = run_res {
        log::error!("{:#?}", err);
        eprintln!("{}", err.to_string().red());
        eprintln!("{}", err.backtrace());
        process::exit(1);
    }
}
