use crate::traits::FromBytes;
use crate::protocol::GroupParams;
use lazy_static::lazy_static;
use num_bigint::BigUint;
#[allow(unused_imports)]
use num_traits::FromBytes as NumFromBytes;
use pasta_curves::pallas::Point as PallasPoint;
use pasta_curves::vesta::Point as VestaPoint;
use std::str::FromStr;

lazy_static! {
    pub static ref DLOG_GROUP_PARAMS: GroupParams<BigUint> = {
        GroupParams {
            p: BigUint::from_str("42765216643065397982265462252423826320512529931694366715111734768493812630447").unwrap(),
            q: BigUint::from_str("21382608321532698991132731126211913160256264965847183357555867384246906315223").unwrap(),
            g: BigUint::from_str("4").unwrap(),
            h: BigUint::from_str("9").unwrap(),
        }
    };
}

fn convert(vec: &Vec<u8>) -> Result<&[u8; 32], &'static str> {
    if vec.len() == 32 {
        let slice: &[u8; 32] = vec
            .as_slice()
            .try_into()
            .expect("Wrong length");
        Ok(slice)
    } else {
        Err("Length != 32")
    }
}

