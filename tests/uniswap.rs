#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use substreams_abis::evm::uniswap::v2::uniswapv2pair::events::Sync;
    use substreams_ethereum::pb::eth::v2::Log;
    use substreams::hex;
    use substreams::scalar::BigInt;

    #[test]
    fn test_uniswap_sync() {
        // https://etherscan.io/tx/0x9bec174ed8e96e6fe302e38a1a62ea4660830d1e791029048367be0caf063573#eventlog
        let log = Log {
            // Sync liquidity pairs
            // reserve0: 1149216704419334
            // reserve1: 1935417248994
            topics: vec![
                hex!("1c411e9a96e071241c2f21f7726b17ae89e3cab4c78be50e062b03a9fffbbad1").to_vec(), // topic0
            ],
            data: hex!("00000000000000000000000000000000000000000000000000041534dd6fd606000000000000000000000000000000000000000000000000000001c29fdb8ce2").to_vec(), // 199962525
            address: hex!("fCd13EA0B906f2f87229650b8D93A51B2e839EBD").to_vec(), // Tether USDT
            block_index: 0,
            index: 0,
            ordinal: 0,
        };

        match Sync::decode(&log) {
            Ok(event) => {
                assert_eq!(event.reserve0, BigInt::from_str("1149216704419334").unwrap());
                assert_eq!(event.reserve1, BigInt::from_str("1935417248994").unwrap());
            }
            Err(e) => {
                panic!("Error decoding Transfer event: {:?}", e);
            }
        }
    }
}