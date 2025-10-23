#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use substreams::hex;
    use substreams::scalar::BigInt;
    use substreams_abis::evm::uniswap::v2::factory::events::PairCreated;
    use substreams_abis::evm::uniswap::v2::pair::events::Sync;
    use substreams_ethereum::pb::eth::v2::Log;

    #[test]
    fn test_uniswap_pair() {
        // https://etherscan.io/tx/0x9bec174ed8e96e6fe302e38a1a62ea4660830d1e791029048367be0caf063573#eventlog
        let log = Log {
            // Sync liquidity pairs
            // reserve0: 1149216704419334
            // reserve1: 1935417248994
            topics: vec![
                hex!("1c411e9a96e071241c2f21f7726b17ae89e3cab4c78be50e062b03a9fffbbad1").to_vec(), // topic0
            ],
            data: hex!("00000000000000000000000000000000000000000000000000041534dd6fd606000000000000000000000000000000000000000000000000000001c29fdb8ce2").to_vec(), // 199962525
            address: hex!("fCd13EA0B906f2f87229650b8D93A51B2e839EBD").to_vec(), // Uniswap V2: DOGE-USDT
            block_index: 0,
            index: 0,
            ordinal: 0,
        };

        match Sync::decode(&log) {
            Ok(event) => {
                assert_eq!(
                    event.reserve0,
                    BigInt::from_str("1149216704419334").unwrap()
                );
                assert_eq!(event.reserve1, BigInt::from_str("1935417248994").unwrap());
            }
            Err(e) => {
                panic!("Error decoding Transfer event: {:?}", e);
            }
        }
    }

    #[test]
    fn test_uniswap_factory() {
        // https://etherscan.io/tx/0x245184375c2f1a6117bd8298a8b6fefcb76ff682e6b2c210fdb19d71514488e9
        let log = Log {
            topics: vec![
                hex!("0d3648bd0f6ba80134a33ba9275ac585d9d315f0ad8355cddefde31afa28d0e9").to_vec(), // topic0
                hex!("0000000000000000000000005b51b4cfa71d5f9dac92d1333810b6e9e4186da2").to_vec(), // topic1
                hex!("000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").to_vec(), // topic2
            ],
            data: hex!("000000000000000000000000ce89ffd7a589987cf785ff13f70360dc05f7bf6b0000000000000000000000000000000000000000000000000000000000063934").to_vec(), // 199962525
            address: hex!("5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f").to_vec(), // Uniswap V2: Factory Contract
            block_index: 0,
            index: 0,
            ordinal: 0,
        };

        match PairCreated::decode(&log) {
            Ok(event) => {
                assert_eq!(event.pair, hex!("ce89ffd7a589987cf785ff13f70360dc05f7bf6b"));
            }
            Err(e) => {
                panic!("Error decoding Transfer event: {:?}", e);
            }
        }
    }
}
