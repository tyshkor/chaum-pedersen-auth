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

