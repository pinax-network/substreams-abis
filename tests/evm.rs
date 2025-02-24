#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use substreams_abis::evm::token::erc20::events::Transfer;
    use substreams_ethereum::pb::eth::v2::Log;
    use substreams::{hex, scalar::BigInt};

    #[test]
    fn test_erc20_transfer() {
        // https://etherscan.io/tx/0xa772f5f06f08177fd74fee1e06608e9dd7cc1cd7ae9716995137554c01a52f35#eventlog#1
        let log = Log {
            // Provide the signature for the ERC20 Transfer event:
            // Topic0 is keccak("Transfer(address,address,uint256)")
            topics: vec![
                hex!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef").to_vec(), // topic0
                hex!("0000000000000000000000006d1d1ebe7da598194293784252659e862d55b52c").to_vec(), // topic1
                hex!("000000000000000000000000c7bbec68d12a0d1830360f8ec58fa599ba1b0e9b").to_vec(), // topic2
                vec![], // topic3
            ],
            data: hex!("00000000000000000000000000000000000000000000000000000000caa7e200").to_vec(), // 199962525
            address: hex!("dac17f958d2ee523a2206206994597c13d831ec7").to_vec(), // Tether USDT
            block_index: 0,
            index: 0,
            ordinal: 0,
        };

        match Transfer::decode(&log) {
            Ok(transfer) => {
                assert_eq!(transfer.from, hex!("6D1D1ebe7dA598194293784252659e862d55b52c"));
                assert_eq!(transfer.to,   hex!("c7bBeC68d12a0d1830360F8Ec58fA599bA1b0e9b"));
                assert_eq!(transfer.value, BigInt::from_str("3400000000").unwrap());
            }
            Err(e) => {
                panic!("Error decoding Transfer event: {:?}", e);
            }
        }
    }
}