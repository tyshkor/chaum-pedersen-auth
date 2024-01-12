pub mod cli;
pub mod service;
pub mod utils;

use crate::cli::Cli;
use crate::utils::hash_or_generate_random;
use chaum_pedersen::enums::{EllipticCurve, Flavor};
use chaum_pedersen::protocol::constants::DLOG_GROUP_PARAMS;
use chaum_pedersen::protocol::constants::PALLAS_GROUP_PARAMS;
use chaum_pedersen::protocol::constants::VESTA_GROUP_PARAMS;
use chaum_pedersen::protocol::{
    discrete_log::DiscreteLog, elliptic_curves::pallas::PallasEllipticCurve,
    elliptic_curves::vesta::VestaEllipticCurve,
};
use service::run_protocol;
use service::AuthClientService;
use structopt::StructOpt;

/// This starts a client to interact with a server implementing the Chaum-Pedersen protocol through a CLI.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Cli {
        host,
        port,
        flavor,
        curve,
        user,
        secret,
    } = Cli::from_args();

    println!(" Starting client ");
    println!("      host: {}", host);
    println!("      port: {}", port);
    println!("      flavor: {}", flavor);
    if flavor == Flavor::EllipticCurve {
        println!("      elliptic curve: {}", curve)
    }
    println!("      user: {}", user);

    let mut client = AuthClientService::connect(format!("http://{}:{}", host, port)).await?;
    match flavor {
        Flavor::DiscreteLog => {
            let dl_params = &DLOG_GROUP_PARAMS;
            run_protocol::<DiscreteLog, _, _>(
                &dl_params,
                &hash_or_generate_random(secret.as_ref())?,
                &user,
                &mut client,
            )
            .await?
        }
        Flavor::EllipticCurve => match curve {
            EllipticCurve::Pallas => {
                let ec_params = PALLAS_GROUP_PARAMS.to_owned();
                run_protocol::<PallasEllipticCurve, _, _>(
                    &ec_params,
                    &hash_or_generate_random(secret.as_ref())?,
                    &user,
                    &mut client,
                )
                .await?
            }

            EllipticCurve::Vesta => {
                let ec_params = VESTA_GROUP_PARAMS.to_owned();
                run_protocol::<VestaEllipticCurve, _, _>(
                    &ec_params,
                    &hash_or_generate_random(secret.as_ref())?,
                    &user,
                    &mut client,
                )
                .await?
            }
        },
    }
    Ok(())
}
