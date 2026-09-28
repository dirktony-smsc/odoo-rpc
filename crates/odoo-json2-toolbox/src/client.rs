use std::{fs::File, io::Read, path::Path};

use odoo_json2::OdooJson2Client;
use reqwest::{Certificate, ClientBuilder};

use crate::config::ConfigFile;

pub(crate) fn create_client(config: ConfigFile) -> anyhow::Result<OdooJson2Client> {
    let mut reqwest_builder = ClientBuilder::new();
    if let Some(ssl) = config.ssl_config {
        for cert_path in ssl.additional_certs {
            let cert_path = Path::new(&cert_path).canonicalize()?;
            log::debug!("importing cert `{:?}`", cert_path);
            let mut buf = Vec::<u8>::new();
            File::open(cert_path)?.read_to_end(&mut buf)?;
            reqwest_builder = reqwest_builder.tls_certs_merge(Certificate::from_pem_bundle(&buf)?);
        }
    }

    let builder = {
        let config = config.odoo;
        let mut builder = OdooJson2Client::builder()
            .base_url(config.url)
            .api_key(config.api_key);
        if let Some(database) = config.database {
            builder = builder.database(database);
        }
        if let Some(host) = config.host {
            builder = builder.host(host);
        }
        if let Some(user_agent) = config.user_agent {
            builder = builder.user_agent(user_agent);
        };
        builder
    };

    Ok(builder.reqwest_client_builder(reqwest_builder).build()?)
}
