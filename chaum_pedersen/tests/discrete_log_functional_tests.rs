use chaum_pedersen::protocol::constants::DLOG_GROUP_PARAMS;
use chaum_pedersen::protocol::discrete_log::DiscreteLog;
use chaum_pedersen::protocol::{GroupParams, Protocol};
use chaum_pedersen::traits::Random;
use num_bigint::{BigUint, RandBigInt};
use rand::rngs::OsRng;

mod run;

use run::run_protocol;

#[test]
fn dlog_success() {
    let params = DLOG_GROUP_PARAMS.to_owned();
    let mut rng = OsRng;
    let x = rng.gen_biguint_below(&params.p);
    assert!(run_protocol::<DiscreteLog>(&params, &x));
}

#[test]
fn dlog_fail() {
    let params = DLOG_GROUP_PARAMS.to_owned();
    let mut rng = OsRng;
    let x = rng.gen_biguint_below(&params.p);
    let (cp, _) = DiscreteLog::commitment(&params, &x);
    let c = DiscreteLog::challenge(&params);
    let fake_response = BigUint::random().unwrap();
    let verified = DiscreteLog::verify(&params, &fake_response, &c, &cp);
    assert!(!verified);
}

#[test]
fn test_verify() {
    let g = <BigUint as From<u32>>::from(4u32);
    let h = <BigUint as From<u32>>::from(9u32);
    let p = <BigUint as From<u32>>::from(23u32);
    let q = <BigUint as From<u32>>::from(11u32);
    let params = GroupParams::<BigUint> {
        g: g.clone(),
        h: h.clone(),
        p: p.clone(),
        q: q.clone(),
    };
    let cp = (
        <BigUint as From<u32>>::from(6u32),
        <BigUint as From<u32>>::from(18u32),
        <BigUint as From<u32>>::from(2u32),
        <BigUint as From<u32>>::from(3u32),
    );
    // client calculates response
    let x = <BigUint as From<u32>>::from(10u32);
    let k = <BigUint as From<u32>>::from(17u32);
    let c = <BigUint as From<u32>>::from(0u32);
    let s = DiscreteLog::challenge_response(&params, &k, &c, &x);
    // server verifies
    assert!(DiscreteLog::verify(&params, &s, &c, &cp));
}
