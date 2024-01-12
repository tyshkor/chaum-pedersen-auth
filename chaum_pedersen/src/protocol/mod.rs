pub mod constants;
pub mod discrete_log;
pub mod elliptic_curves;

/// A struct representing parameters of groups used in implementation.
#[derive(Copy, Clone, Debug)]
pub struct GroupParams<T> {
    /// The generator `g` of the group.
    pub g: T,
    /// An additional generator `h`, independent from `g`.
    pub h: T,
    /// The prime modulus `p` defining the size of the group.
    pub p: T,
    /// The order `q` of the subgroup generated by `g` and `h`.
    pub q: T,
}

/// A trait defining the interface for the Chaum-Pedersen zero-knowledge protocol.
pub trait Protocol {
    type Secret;
    type Response;
    type Challenge;
    type GroupParameters;
    type CommitParameters;
    type CommitmentRandom;

    /// Calculates the commitment in the Chaum-Pedersen protocol.
    fn commitment(
        params: &Self::GroupParameters, x: &Self::Secret,
    ) -> (Self::CommitParameters, Self::CommitmentRandom)
    where
        Self: Sized;

    /// Generates a challenge in the Chaum-Pedersen protocol.
    fn challenge(params: &Self::GroupParameters) -> Self::Challenge
    where
        Self: Sized;

    fn challenge_response(
        params: &Self::GroupParameters, k: &Self::CommitmentRandom, c: &Self::Challenge,
        x: &Self::Secret,
    ) -> Self::Response
    where
        Self: Sized;

    fn verify(
        params: &Self::GroupParameters, s: &Self::Response, c: &Self::Challenge,
        cp: &Self::CommitParameters,
    ) -> bool
    where
        Self: Sized;
}
