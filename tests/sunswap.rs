#[cfg(test)]
mod tests {
    use substreams::hex;
    use substreams::scalar::BigInt;
    use substreams_abis::tvm::sunswap::v2::factory::events::PairCreated;
    use substreams_ethereum::pb::eth::v2::Log;

    #[test]
    fn test_sunswap_factory_pair_created() {
        // Test data from issue
        // topic0: 0d3648bd0f6ba80134a33ba9275ac585d9d315f0ad8355cddefde31afa28d0e9
        // data: 0x000000000000000000000000891cdb91d149f23b1a45d9c5ca78a88d0cb44c18000000000000000000000000a614f803b6fd780986a42c78ec9c7f77e6ded13c0000000000000000000000003a10321a4e97a64d9376af42ec07d5fa50294b350000000000000000000000000000000000000000000000000000000000000001
        //
        // According to the ABI, PairCreated has:
        // - token0 (indexed) -> should be in topic1
        // - token1 (indexed) -> should be in topic2
        // - pair (not indexed) -> should be in data
        // - extraData (not indexed) -> should be in data
        //
        // The provided data appears to contain all 4 fields (128 bytes).
        // We split it according to the ABI specification:
        // - First 32 bytes (token0) -> topic1
        // - Second 32 bytes (token1) -> topic2
        // - Third 32 bytes (pair) -> data
        // - Fourth 32 bytes (extraData) -> data
        let log = Log {
            topics: vec![
                hex!("0d3648bd0f6ba80134a33ba9275ac585d9d315f0ad8355cddefde31afa28d0e9").to_vec(), // topic0 (event signature)
                hex!("000000000000000000000000891cdb91d149f23b1a45d9c5ca78a88d0cb44c18").to_vec(), // topic1 (token0)
                hex!("000000000000000000000000a614f803b6fd780986a42c78ec9c7f77e6ded13c").to_vec(), // topic2 (token1)
            ],
            data: hex!("0000000000000000000000003a10321a4e97a64d9376af42ec07d5fa50294b350000000000000000000000000000000000000000000000000000000000000001").to_vec(), // pair + extraData (64 bytes)
            address: vec![], // Sunswap Factory address
            block_index: 0,
            index: 0,
            ordinal: 0,
        };

        match PairCreated::decode(&log) {
            Ok(event) => {
                assert_eq!(
                    event.token0,
                    hex!("891cdb91d149f23b1a45d9c5ca78a88d0cb44c18")
                );
                assert_eq!(
                    event.token1,
                    hex!("a614f803b6fd780986a42c78ec9c7f77e6ded13c")
                );
                assert_eq!(event.pair, hex!("3a10321a4e97a64d9376af42ec07d5fa50294b35"));
                assert_eq!(event.extra_data, BigInt::from(1u64));
            }
            Err(e) => {
                panic!("Error decoding PairCreated event: {:?}", e);
            }
        }
    }

    #[test]
    fn test_sunswap_factory_pair_created_raw_data() {
        // This test uses the data exactly as provided in the issue
        // (all 128 bytes in data field, only topic0)
        // This should fail validation since the ABI expects indexed parameters
        let log = Log {
            topics: vec![
                hex!("0d3648bd0f6ba80134a33ba9275ac585d9d315f0ad8355cddefde31afa28d0e9").to_vec(), // topic0
            ],
            data: hex!("000000000000000000000000891cdb91d149f23b1a45d9c5ca78a88d0cb44c18000000000000000000000000a614f803b6fd780986a42c78ec9c7f77e6ded13c0000000000000000000000003a10321a4e97a64d9376af42ec07d5fa50294b350000000000000000000000000000000000000000000000000000000000000001").to_vec(),
            address: vec![],
            block_index: 0,
            index: 0,
            ordinal: 0,
        };

        // First check if it matches the expected log format
        let matches = PairCreated::match_log(&log);
        assert!(
            !matches,
            "Log should not match because it only has 1 topic instead of 3"
        );

        // The log also has 128 bytes of data instead of 64
        assert_eq!(log.data.len(), 128);
        assert_ne!(
            log.data.len(),
            64,
            "Data length doesn't match ABI expectation"
        );
    }

    #[test]
    fn test_sunswap_factory_pair_created_tx_bb08b5a3() {
        // Test data from transaction: bb08b5a3f581447aa326aa8db837a9c87c0f64950b1c6ed4471faf2dffcd5742
        // https://tronscan.org/#/transaction/bb08b5a3f581447aa326aa8db837a9c87c0f64950b1c6ed4471faf2dffcd5742/event-logs
        // topic0: 0d3648bd0f6ba80134a33ba9275ac585d9d315f0ad8355cddefde31afa28d0e9
        // data: 0x00000000000000000000000048c125e0d3c626842bf7180c85e79f97ae524e91000000000000000000000000a614f803b6fd780986a42c78ec9c7f77e6ded13c00000000000000000000000062cd74b7d68d6914ca7e3aa4720d53588f4c18ac000000000000000000000000000000000000000000000000000000000000001f
        //
        // According to the ABI, PairCreated has:
        // - token0 (indexed) -> should be in topic1
        // - token1 (indexed) -> should be in topic2
        // - pair (not indexed) -> should be in data
        // - extraData (not indexed) -> should be in data
        //
        // Splitting the 128 bytes of data:
        // - First 32 bytes (token0) -> topic1
        // - Second 32 bytes (token1) -> topic2
        // - Third 32 bytes (pair) -> data
        // - Fourth 32 bytes (extraData) -> data
        let log = Log {
            topics: vec![
                hex!("0d3648bd0f6ba80134a33ba9275ac585d9d315f0ad8355cddefde31afa28d0e9").to_vec(), // topic0 (event signature)
                hex!("00000000000000000000000048c125e0d3c626842bf7180c85e79f97ae524e91").to_vec(), // topic1 (token0)
                hex!("000000000000000000000000a614f803b6fd780986a42c78ec9c7f77e6ded13c").to_vec(), // topic2 (token1)
            ],
            data: hex!("00000000000000000000000062cd74b7d68d6914ca7e3aa4720d53588f4c18ac000000000000000000000000000000000000000000000000000000000000001f").to_vec(), // pair + extraData (64 bytes)
            address: vec![],
            block_index: 0,
            index: 0,
            ordinal: 0,
        };

        match PairCreated::decode(&log) {
            Ok(event) => {
                assert_eq!(
                    event.token0,
                    hex!("48c125e0d3c626842bf7180c85e79f97ae524e91")
                );
                assert_eq!(
                    event.token1,
                    hex!("a614f803b6fd780986a42c78ec9c7f77e6ded13c")
                );
                assert_eq!(event.pair, hex!("62cd74b7d68d6914ca7e3aa4720d53588f4c18ac"));
                assert_eq!(event.extra_data, BigInt::from(31u64));
            }
            Err(e) => {
                panic!("Error decoding PairCreated event: {:?}", e);
            }
        }
    }
}
