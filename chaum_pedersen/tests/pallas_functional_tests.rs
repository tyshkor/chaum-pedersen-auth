use chaum_pedersen::protocol::constants::PALLAS_GROUP_PARAMS;
use chaum_pedersen::protocol::elliptic_curves::pallas::PallasEllipticCurve;
use chaum_pedersen::protocol::Protocol;
use pasta_curves::group::ff::Field;
use pasta_curves::group::GroupEncoding;
use pasta_curves::pallas::{Point, Scalar};
use rand_core::OsRng;

mod run;

use run::run_protocol;

#[test]
fn pallas_success_verification() {
    let mut rng = OsRng;
    let x = <Scalar as Field>::random(&mut rng);
    let params = PALLAS_GROUP_PARAMS.to_owned();
    let gb = params.g.to_bytes();
    let restored_g = Point::from_bytes(&gb).unwrap();
    assert_eq!(params.g, restored_g);
    let hb = params.h.to_bytes();
    let restored_h = Point::from_bytes(&hb).unwrap();
    assert_eq!(params.h, restored_h);
    assert!(run_protocol::<PallasEllipticCurve>(&params, &x));
}

#[test]
fn pallas_fail_verification() {
    let mut rng = OsRng;
    let x = <Scalar as Field>::random(&mut rng);
    let params = PALLAS_GROUP_PARAMS.to_owned();
    let (cp, _) = PallasEllipticCurve::commitment(&params, &x);
    let c = PallasEllipticCurve::challenge(&params);
    let fake_response = <Scalar as Field>::random(&mut rng);
    // Asserting that the verification should fail with the fake response.
    let verified = PallasEllipticCurve::verify(&params, &fake_response, &c, &cp);
    assert!(!verified);
}
