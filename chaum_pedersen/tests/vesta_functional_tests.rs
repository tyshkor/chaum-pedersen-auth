use chaum_pedersen::protocol::elliptic_curves::vesta::VestaEllipticCurve;
use chaum_pedersen::protocol::Protocol;
use pasta_curves::group::ff::Field;
use pasta_curves::group::GroupEncoding;
use pasta_curves::vesta::Point;
use pasta_curves::vesta::Scalar;
use rand_core::OsRng;

mod run;

use chaum_pedersen::protocol::constants::VESTA_GROUP_PARAMS;
use run::run_protocol;

#[test]
fn vesta_success_verification() {
    let mut rng = OsRng;
    let x = <Scalar as Field>::random(&mut rng);
    let params = VESTA_GROUP_PARAMS.to_owned();
    let hb = params.h.to_bytes();
    let restored_h = Point::from_bytes(&hb).unwrap();
    assert_eq!(params.h, restored_h);
    assert!(run_protocol::<VestaEllipticCurve>(&params, &x));
}

#[test]
fn vesta_fail_verification() {
    let mut rng = OsRng;
    let x = <Scalar as Field>::random(&mut rng);
    let params = VESTA_GROUP_PARAMS.to_owned();
    let (cp, _) = VestaEllipticCurve::commitment(&params, &x);
    let c = VestaEllipticCurve::challenge(&params);
    let fake_response = <Scalar as Field>::random(&mut rng);
    // Asserting that the verification should fail with the fake response.
    let verified = VestaEllipticCurve::verify(&params, &fake_response, &c, &cp);
    assert!(!verified);
}
