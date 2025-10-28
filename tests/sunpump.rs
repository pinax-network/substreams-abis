#[cfg(test)]
mod tests {
    use substreams::hex;
    use substreams::scalar::BigInt;
    use substreams_abis::tvm::sunpump::v1::launchpad::events::TokenCreateV2;
    use substreams_ethereum::pb::eth::v2::Log;
    use substreams_ethereum::Event;

    // Topic ID verification:
    // Event signature: TokenCreateV2(address,address,uint256,uint256,string,string)
    // keccak256 hash: 0x7d3561bb6c41a7796f0b6a9b463b4be53333e86339005c596fd4e5f53c9cc8f5
    // This matches TokenCreateV2::TOPIC_ID constant defined in the generated code

    #[test]
    fn test_sunpump_token_create_decode_only() {
        // This test demonstrates that decode() only looks at log.data and ignores topics
        // Note: The topic here (16624d4e...) does NOT match TokenCreateV2's actual topic (7d3561bb...)
        // but decode() still works because it doesn't validate the topic
        //
        // log: trx = bccd5365674cd7b193adbbb37b247bd84d41ae5c73be98ac3fc4150c2a1369fb
        // log: log.address = TEPcBKJB7N6rF9xKQKPVeSrscsRTfsVFVi
        // log: log.topics[0] = 16624d4e070855bf4f06e05f0fc8f60958fbb7fc14336e1d4f94df210e2d585a
        // log: log.data =
        // 000000000000000000000000afa761010e3d6180661ff902a70a265733d3b1f600000000000000000000000092ad9f5f8d9750e0
        // 3b310bbc2712121d8785c5360000000000000000000000000000000000000000000000000000000000000064000000000000000000000000000
        // 000000000000000000000000000000098968000000000000000000000000000000000000000000000000000000000000000c000000000000000
        // 0000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000095
        // 4726f6e5f42756c6c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
        // 00000000000442554c4c00000000000000000000000000000000000000000000000000000000

        let log = Log {
            topics: vec![
                hex!("16624d4e070855bf4f06e05f0fc8f60958fbb7fc14336e1d4f94df210e2d585a").to_vec(), // WRONG topic - not TokenCreateV2
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

        // decode() works even with wrong topic because it only checks log.data
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

    #[test]
    fn test_sunpump_token_create_v2_match_and_decode_success() {
        // This test demonstrates match_and_decode() working correctly with the RIGHT topic
        // Event signature: TokenCreateV2(address,address,uint256,uint256,string,string)
        // keccak256(signature) = 0x7d3561bb6c41a7796f0b6a9b463b4be53333e86339005c596fd4e5f53c9cc8f5
        // This matches the TOPIC_ID constant in TokenCreateV2 event definition

        let log = Log {
            topics: vec![
                hex!("7d3561bb6c41a7796f0b6a9b463b4be53333e86339005c596fd4e5f53c9cc8f5").to_vec(), // CORRECT TokenCreateV2 topic
            ],
            data: hex!(
                "000000000000000000000000afa761010e3d6180661ff902a70a265733d3b1f600000000000000000000000092ad9f5f8d9750e03b310bbc2712121d8785c5360000000000000000000000000000000000000000000000000000000000000064000000000000000000000000000000000000000000000000000000000098968000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000954726f6e5f42756c6c0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000442554c4c00000000000000000000000000000000000000000000000000000000"
            )
            .to_vec(),
            address: vec![],
            block_index: 123,
            index: 0,
            ordinal: 0,
        };

        // match_and_decode() should work now because topic matches
        match TokenCreateV2::match_and_decode(&log) {
            Some(event) => {
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
            None => {
                panic!("match_and_decode should have succeeded with correct topic");
            }
        }
    }

    #[test]
    fn test_sunpump_token_create_v2_match_and_decode_fails_on_wrong_topic() {
        // This test demonstrates that match_and_decode() correctly rejects logs with wrong topics
        // This is the SAFE behavior - it validates both the topic AND the data

        let log = Log {
            topics: vec![
                hex!("16624d4e070855bf4f06e05f0fc8f60958fbb7fc14336e1d4f94df210e2d585a").to_vec(), // WRONG topic
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

        // match_and_decode() should return None because topic doesn't match
        let result = TokenCreateV2::match_and_decode(&log);
        assert!(
            result.is_none(),
            "match_and_decode should return None when topic doesn't match TokenCreateV2"
        );
    }

    #[test]
    fn test_sunpump_token_create_v2_match_log() {
        // Test that match_log correctly validates topic and data size

        // Correct log
        let correct_log = Log {
            topics: vec![
                hex!("7d3561bb6c41a7796f0b6a9b463b4be53333e86339005c596fd4e5f53c9cc8f5").to_vec(),
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
        assert!(
            TokenCreateV2::match_log(&correct_log),
            "Should match log with correct topic and data size"
        );

        // Wrong topic
        let wrong_topic_log = Log {
            topics: vec![
                hex!("0000000000000000000000000000000000000000000000000000000000000000").to_vec(),
            ],
            data: correct_log.data.clone(),
            address: vec![],
            block_index: 0,
            index: 0,
            ordinal: 0,
        };
        assert!(
            !TokenCreateV2::match_log(&wrong_topic_log),
            "Should not match log with wrong topic"
        );

        // Wrong data size (too short)
        let wrong_size_log = Log {
            topics: vec![
                hex!("7d3561bb6c41a7796f0b6a9b463b4be53333e86339005c596fd4e5f53c9cc8f5").to_vec(),
            ],
            data: hex!("0000000000000000000000000000000000000000000000000000000000000001").to_vec(),
            address: vec![],
            block_index: 0,
            index: 0,
            ordinal: 0,
        };
        assert!(
            !TokenCreateV2::match_log(&wrong_size_log),
            "Should not match log with incorrect data size"
        );
    }

    // TokenPurchased()
    // log: trx = bccd5365674cd7b193adbbb37b247bd84d41ae5c73be98ac3fc4150c2a1369fb
    // log: log.address = TEPcBKJB7N6rF9xKQKPVeSrscsRTfsVFVi
    // log: log.topics[0] = 63abb62535c21a5d221cf9c15994097b8880cc986d82faf80f57382b998dbae5
    // log: log.topics[1] = 000000000000000000000000afa761010e3d6180661ff902a70a265733d3b1f6
    // log: log.topics[2] = 00000000000000000000000092ad9f5f8d9750e03b310bbc2712121d8785c536
    // log: log.data =
    // 0000000000000000000000000000000000000000000000000000000000970fe00000000000000000000000000000000000000000
    // 0000000000000000000186a00000000000000000000000000000000000000000000040126fe218e61cdc21a2000000000000000000000000000
    // 00000000000000374d53dca7a7050d123de5e

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
