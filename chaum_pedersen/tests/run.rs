use chaum_pedersen::protocol::Protocol;

/// Runs the Chaum-Pedersen protocol using a generic implementation.
pub fn run_protocol<T>(params: &T::GroupParameters, x: &T::Secret) -> bool
where
    T: Protocol,
{
    // The client calculates the commitment.
    let (cp, k) = T::commitment(params, x);

    // The server sends a challenge to the client.
    let c = T::challenge(params);

    // The client calculates the response.
    let s = T::challenge_response(params, &k, &c, &x);

    // The server verifies the response.
    T::verify(params, &s, &c, &cp)
}
