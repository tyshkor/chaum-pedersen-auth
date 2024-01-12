/// Core services and business logic implementation.
pub mod service;

/// CRUD APIs to work with storage.
pub mod apis;

pub mod cli;
pub mod errors;

use anyhow::Result;
use chaum_pedersen::enums::{EllipticCurve, Flavor};
use chaum_pedersen::protocol::constants::{
    DLOG_GROUP_PARAMS, PALLAS_GROUP_PARAMS, VESTA_GROUP_PARAMS,
};
use chaum_pedersen::protocol::discrete_log::DiscreteLog;
use chaum_pedersen::protocol::elliptic_curves::pallas::PallasEllipticCurve;
use chaum_pedersen::protocol::elliptic_curves::vesta::VestaEllipticCurve;
use service::zkp_auth::auth_server::AuthServer;
use service::AuthService;
use structopt::StructOpt;
use tonic::transport::Server;

use crate::cli::Cli;
use crate::errors::CliError;

/// This starts a server implementing the Chaum-Pedersen protocol through a CLI.
#[tokio::main]
async fn main() -> Result<()> {
    let Cli {
        host,
        port,
        flavor,
        curve,
        ..
    } = Cli::from_args();

    println!(" Starting server ");
    println!("       host: {}", host);
    println!("       port: {}", port);
    println!("       flavor: {}", flavor);
    if flavor == Flavor::EllipticCurve {
        println!("       elliptic curve: {}", curve)
    }

    let addr = format!("{}:{}", host, port)
        .parse()
        .map_err(|_| CliError::AddressParsing)?;

    // Initialize and start the server based on flavor.
    match flavor {
        Flavor::DiscreteLog => {
            let params = DLOG_GROUP_PARAMS.to_owned();
            let auth = AuthService::<DiscreteLog, _, _>::new(params);
            Server::builder()
                .add_service(AuthServer::new(auth))
                .serve(addr)
                .await?;
        }
        Flavor::EllipticCurve => match curve {
            EllipticCurve::Pallas => {
                let params = PALLAS_GROUP_PARAMS.to_owned();
                let auth = AuthService::<PallasEllipticCurve, _, _>::new(params);
                Server::builder()
                    .add_service(AuthServer::new(auth))
                    .serve(addr)
                    .await?;
            }

            EllipticCurve::Vesta => {
                let params = VESTA_GROUP_PARAMS.to_owned();
                let auth = AuthService::<VestaEllipticCurve, _, _>::new(params);
                Server::builder()
                    .add_service(AuthServer::new(auth))
                    .serve(addr)
                    .await?;
            }
        },
    }

    Ok(())
}
