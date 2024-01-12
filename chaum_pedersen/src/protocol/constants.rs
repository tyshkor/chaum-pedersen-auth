use crate::traits::FromBytes;
use crate::protocol::GroupParams;
use lazy_static::lazy_static;
use num_bigint::BigUint;
#[allow(unused_imports)]
use num_traits::FromBytes as NumFromBytes;
use pasta_curves::pallas::Point as PallasPoint;
use pasta_curves::vesta::Point as VestaPoint;
use std::str::FromStr;


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

