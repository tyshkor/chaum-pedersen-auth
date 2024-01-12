use structopt::StructOpt;
use strum::VariantNames;
use chaum_pedersen::enums::{Flavor, EllipticCurve};

#[derive(Debug, StructOpt)]
#[structopt(name = "client", about = "A client for the ZKPass server")]
pub struct Cli {
    #[structopt(short, long, default_value = "[::1]")]
    pub host: String,
    #[structopt(short, long, default_value = "50051")]
    pub port: i32,
    #[structopt(short, long)]
    pub secret: Option<String>,
    #[structopt(short, long, default_value = "peggy")]
    pub user: String,
    /// Underlying type of the Chaum-Pedersen protocol to use.
    #[structopt(short, long, possible_values = Flavor::VARIANTS, default_value = "discrete_log")]
    pub flavor: Flavor,
    /// Elliptic curve type for the Elliptic Curve implementation.
    #[structopt(short, long, possible_values = EllipticCurve::VARIANTS, default_value = "pallas", required_if("flavor", "elliptic_curve"))]
    pub curve: EllipticCurve,
}

