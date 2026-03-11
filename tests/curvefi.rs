#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use substreams::{hex, scalar::BigInt};
    use substreams_abis::dex::curvefi::stableswap::constructor::Constructor;

    #[test]
    fn test_stableswap_constructor_decode() {
        let encoded = ethabi::encode(&[
            ethabi::Token::Address(ethabi::Address::from_slice(
                hex!("1111111111111111111111111111111111111111").as_ref(),
            )),
            ethabi::Token::FixedArray(vec![
                ethabi::Token::Address(ethabi::Address::from_slice(
                    hex!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").as_ref(),
                )),
                ethabi::Token::Address(ethabi::Address::from_slice(
                    hex!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb").as_ref(),
                )),
                ethabi::Token::Address(ethabi::Address::from_slice(
                    hex!("cccccccccccccccccccccccccccccccccccccccc").as_ref(),
                )),
            ]),
            ethabi::Token::Address(ethabi::Address::from_slice(
                hex!("2222222222222222222222222222222222222222").as_ref(),
            )),
            ethabi::Token::Uint(ethabi::Uint::from(2000u64)),
            ethabi::Token::Uint(ethabi::Uint::from(4_000_000u64)),
            ethabi::Token::Uint(ethabi::Uint::from(5_000_000_000u64)),
        ]);

        let constructor = Constructor::decode(&encoded).expect("constructor should decode");

        assert_eq!(constructor.owner, hex!("1111111111111111111111111111111111111111"));
        assert_eq!(
            constructor.coins,
            [
                hex!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                hex!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"),
                hex!("cccccccccccccccccccccccccccccccccccccccc"),
            ]
        );
        assert_eq!(
            constructor.pool_token,
            hex!("2222222222222222222222222222222222222222")
        );
        assert_eq!(constructor.a, BigInt::from_str("2000").unwrap());
        assert_eq!(constructor.fee, BigInt::from_str("4000000").unwrap());
        assert_eq!(
            constructor.admin_fee,
            BigInt::from_str("5000000000").unwrap()
        );
        assert_eq!(constructor.encode(), encoded);
    }
}
