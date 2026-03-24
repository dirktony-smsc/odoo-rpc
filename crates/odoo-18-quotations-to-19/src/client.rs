use std::{fs::File, io::Read, path::Path};

use odoo_json2::OdooJson2Client;
use odoo_rpc::OdooJsonRPCClient;
use reqwest::{Certificate, ClientBuilder};

use crate::{config::Config, error};

#[derive(Debug)]
pub struct Clients {
    pub odoo_18: OdooJsonRPCClient,
    pub odoo_19: OdooJson2Client,
}

impl Clients {
    pub async fn from_config(config: Config) -> Result<Self, error::Error> {
        let mut reqwest_builder = ClientBuilder::new();
        if let Some(ssl) = config.ssl {
            for cert_path in ssl.additional_certs {
                let cert_path = Path::new(&cert_path).canonicalize()?;
                log::debug!("importing cert `{:?}`", cert_path);
                let mut buf = Vec::<u8>::new();
                File::open(cert_path)?.read_to_end(&mut buf)?;
                for cert in Certificate::from_pem_bundle(&buf)? {
                    reqwest_builder = reqwest_builder.add_root_certificate(cert);
                }
            }
        }
        let odoo_18: OdooJsonRPCClient = {
            let odoo_18_cfg = config.odoo_18;
            OdooJsonRPCClient::new(
                odoo_18_cfg.url,
                odoo_18_cfg.user,
                odoo_18_cfg.password,
                odoo_18_cfg.database,
            )
            .await?
        };
        let odoo_19: OdooJson2Client = {
            let odoo_19_cfg = config.odoo_19;
            let mut builder = OdooJson2Client::builder()
                .api_key(odoo_19_cfg.api_key)
                .base_url(odoo_19_cfg.url);
            if let Some(database) = odoo_19_cfg.database {
                builder = builder.database(database);
            }
            if let Some(host) = odoo_19_cfg.host {
                builder = builder.host(host);
            }
            if let Some(user_agent) = odoo_19_cfg.user_agent {
                builder = builder.user_agent(user_agent);
            }
            builder.reqwest_client_builder(reqwest_builder).build()?
        };
        Ok(Clients { odoo_18, odoo_19 })
    }
}
