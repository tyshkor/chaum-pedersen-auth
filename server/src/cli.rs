use chaum_pedersen::enums::{EllipticCurve, Flavor};
use structopt::StructOpt;
use strum::VariantNames;

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(short, long, default_value = "[::1]")]
    pub host: String,
    #[structopt(short, long, default_value = "50051")]
    pub port: i32,
    /// Flavor of the Chaum-Pedersen protocol.
    #[structopt(short, long, possible_values = Flavor::VARIANTS, default_value = "discrete_log")]
    pub flavor: Flavor,
    /// Elliptic curve type, required if one is using elliptic_curve flavor.
    #[structopt(short, long, possible_values = EllipticCurve::VARIANTS, default_value = "pallas", required_if("flavor", "elliptic_curve"))]
    pub curve: EllipticCurve,
}
