/// An enumeration representing the implemented flavors of Chaum-Pedersen protocols.
#[derive(PartialEq, Debug, strum::EnumString, strum::EnumVariantNames, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum Flavor {
    DiscreteLog,
    EllipticCurve,
}

