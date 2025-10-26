#[cfg(test)]
mod tests {
    use substreams::hex;
    use substreams::scalar::BigInt;
    use substreams_abis::tvm::sunpump::v1::launchpad::events::TokenCreateV2;
    use substreams_ethereum::pb::eth::v2::Log;

    #[test]
    fn test_sunpump_token_create() {
        let log = Log {
            topics: vec![
                hex!("16624d4e070855bf4f06e05f0fc8f60958fbb7fc14336e1d4f94df210e2d585a").to_vec(), // topic0 (event signature)
            ],
            data: hex!(
                "000000000000000000000000afa761010e3d6180661ff902a70a265733d3b1f600000000000000000000000092ad9f5f8d9750e03b310bbc2712121d8785c5360000000000000000000000000000000000000000000000000000000000000064000000000000000000000000000000000000000000000000000000000098968000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000954726f6e5f42756c6c0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000442554c4c00000000000000000000000000000000000000000000000000000000"
            )
            .to_vec(),
            address: vec![],
            block_index: 0,
            index: 0,
            ordinal: 0,
        };

        match TokenCreateV2::decode(&log) {
            Ok(event) => {
                assert_eq!(
                    event.token_address,
                    hex!("afa761010e3d6180661ff902a70a265733d3b1f6"),
                    "Token address should match"
                );
                assert_eq!(
                    event.token_index,
                    BigInt::from(100),
                    "Token index should be 100"
                );
                assert_eq!(
                    event.creator,
                    hex!("92ad9f5f8d9750e03b310bbc2712121d8785c536"),
                    "Creator address should match"
                );
                assert_eq!(
                    event.initial_supply,
                    BigInt::from(10_000_000),
                    "Initial supply should be 10,000,000"
                );
                assert_eq!(event.name, "Tron_Bull", "Name should match");
                assert_eq!(event.symbol, "BULL", "Symbol should match");
            }
            Err(e) => {
                panic!("Error decoding TokenCreate event: {:?}", e);
            }
        }
    }

    // #[test]
    // fn test_sunpump_token_create_match_log() {
    //     // Test that match_log correctly identifies a TokenCreate event
    //     let log = Log {
    //         topics: vec![
    //             hex!("1ff0a01c8968e3551472812164f233abb579247de887db8cbb18281c149bee7a").to_vec(),
    //         ],
    //         data: hex!(
    //             "000000000000000000000000891cdb91d149f23b1a45d9c5ca78a88d0cb44c18"
    //             "0000000000000000000000000000000000000000000000000000000000000001"
    //             "000000000000000000000000a614f803b6fd780986a42c78ec9c7f77e6ded13c"
    //         )
    //         .to_vec(),
    //         address: vec![],
    //         block_index: 0,
    //         index: 0,
    //         ordinal: 0,
    //     };

    //     assert!(
    //         TokenCreateV1::match_log(&log),
    //         "Log should match TokenCreate event"
    //     );
    // }

    // #[test]
    // fn test_sunpump_token_create_wrong_topic() {
    //     // Test that match_log rejects a log with wrong topic0
    //     let log = Log {
    //         topics: vec![
    //             hex!("0000000000000000000000000000000000000000000000000000000000000000").to_vec(),
    //         ],
    //         data: hex!(
    //             "000000000000000000000000891cdb91d149f23b1a45d9c5ca78a88d0cb44c18"
    //             "0000000000000000000000000000000000000000000000000000000000000001"
    //             "000000000000000000000000a614f803b6fd780986a42c78ec9c7f77e6ded13c"
    //         )
    //         .to_vec(),
    //         address: vec![],
    //         block_index: 0,
    //         index: 0,
    //         ordinal: 0,
    //     };

    //     assert!(
    //         !TokenCreateV1::match_log(&log),
    //         "Log should not match with wrong topic0"
    //     );
    // }

    // #[test]
    // fn test_sunpump_token_create_wrong_data_length() {
    //     // Test that match_log rejects a log with incorrect data length
    //     let log = Log {
    //         topics: vec![
    //             hex!("1ff0a01c8968e3551472812164f233abb579247de887db8cbb18281c149bee7a").to_vec(),
    //         ],
    //         data: hex!(
    //             "000000000000000000000000891cdb91d149f23b1a45d9c5ca78a88d0cb44c18"
    //             "0000000000000000000000000000000000000000000000000000000000000001"
    //         )
    //         .to_vec(), // Only 64 bytes instead of 96
    //         address: vec![],
    //         block_index: 0,
    //         index: 0,
    //         ordinal: 0,
    //     };

    //     assert!(
    //         !TokenCreateV1::match_log(&log),
    //         "Log should not match with wrong data length"
    //     );
    // }

    // #[test]
    // fn test_sunpump_token_create_large_index() {
    //     // Test with a larger token index value
    //     let log = Log {
    //         topics: vec![
    //             hex!("1ff0a01c8968e3551472812164f233abb579247de887db8cbb18281c149bee7a").to_vec(),
    //         ],
    //         data: hex!(
    //             "00000000000000000000000048c125e0d3c626842bf7180c85e79f97ae524e91"  // tokenAddress
    //             "00000000000000000000000000000000000000000000000000000000000003e7"  // tokenIndex = 999
    //             "00000000000000000000000062cd74b7d68d6914ca7e3aa4720d53588f4c18ac"  // creator
    //         )
    //         .to_vec(),
    //         address: vec![],
    //         block_index: 0,
    //         index: 0,
    //         ordinal: 0,
    //     };

    //     match TokenCreateV1::decode(&log) {
    //         Ok(event) => {
    //             assert_eq!(
    //                 event.token_address,
    //                 hex!("48c125e0d3c626842bf7180c85e79f97ae524e91"),
    //                 "Token address should match"
    //             );
    //             assert_eq!(
    //                 event.token_index,
    //                 BigInt::from(999u64),
    //                 "Token index should be 999"
    //             );
    //             assert_eq!(
    //                 event.creator,
    //                 hex!("62cd74b7d68d6914ca7e3aa4720d53588f4c18ac"),
    //                 "Creator address should match"
    //             );
    //         }
    //         Err(e) => {
    //             panic!("Error decoding TokenCreate event: {:?}", e);
    //         }
    //     }
    // }
}
