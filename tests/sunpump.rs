#[cfg(test)]
mod tests {
    use substreams::hex;
    use substreams::scalar::BigInt;
    use substreams_abis::tvm::sunpump::v1::launchpad::events::TokenCreate;
    use substreams_ethereum::pb::eth::v2::Log;

    #[test]
    fn test_sunpump_token_create() {
        // Test data for TokenCreate event
        // The event has the signature: TokenCreate(address tokenAddress, uint256 tokenIndex, address creator)
        // All parameters are non-indexed, so they should be in the data field
        // Topic0 is the event signature hash: 1ff0a01c8968e3551472812164f233abb579247de887db8cbb18281c149bee7a
        
        // Example event data (96 bytes = 3 parameters * 32 bytes each):
        // - tokenAddress: 0x891cdb91d149f23b1a45d9c5ca78a88d0cb44c18 (padded to 32 bytes)
        // - tokenIndex: 1 (32 bytes)
        // - creator: 0xa614f803b6fd780986a42c78ec9c7f77e6ded13c (padded to 32 bytes)
        let log = Log {
            topics: vec![
                hex!("1ff0a01c8968e3551472812164f233abb579247de887db8cbb18281c149bee7a").to_vec(), // topic0 (event signature)
            ],
            data: hex!(
                "000000000000000000000000891cdb91d149f23b1a45d9c5ca78a88d0cb44c18"  // tokenAddress
                "0000000000000000000000000000000000000000000000000000000000000001"  // tokenIndex = 1
                "000000000000000000000000a614f803b6fd780986a42c78ec9c7f77e6ded13c"  // creator
            ).to_vec(),
            address: vec![],
            block_index: 0,
            index: 0,
            ordinal: 0,
        };

        match TokenCreate::decode(&log) {
            Ok(event) => {
                assert_eq!(
                    event.token_address,
                    hex!("891cdb91d149f23b1a45d9c5ca78a88d0cb44c18"),
                    "Token address should match"
                );
                assert_eq!(
                    event.token_index,
                    BigInt::from(1u64),
                    "Token index should be 1"
                );
                assert_eq!(
                    event.creator,
                    hex!("a614f803b6fd780986a42c78ec9c7f77e6ded13c"),
                    "Creator address should match"
                );
            }
            Err(e) => {
                panic!("Error decoding TokenCreate event: {:?}", e);
            }
        }
    }

    #[test]
    fn test_sunpump_token_create_match_log() {
        // Test that match_log correctly identifies a TokenCreate event
        let log = Log {
            topics: vec![
                hex!("1ff0a01c8968e3551472812164f233abb579247de887db8cbb18281c149bee7a").to_vec(),
            ],
            data: hex!(
                "000000000000000000000000891cdb91d149f23b1a45d9c5ca78a88d0cb44c18"
                "0000000000000000000000000000000000000000000000000000000000000001"
                "000000000000000000000000a614f803b6fd780986a42c78ec9c7f77e6ded13c"
            ).to_vec(),
            address: vec![],
            block_index: 0,
            index: 0,
            ordinal: 0,
        };

        assert!(TokenCreate::match_log(&log), "Log should match TokenCreate event");
    }

    #[test]
    fn test_sunpump_token_create_wrong_topic() {
        // Test that match_log rejects a log with wrong topic0
        let log = Log {
            topics: vec![
                hex!("0000000000000000000000000000000000000000000000000000000000000000").to_vec(),
            ],
            data: hex!(
                "000000000000000000000000891cdb91d149f23b1a45d9c5ca78a88d0cb44c18"
                "0000000000000000000000000000000000000000000000000000000000000001"
                "000000000000000000000000a614f803b6fd780986a42c78ec9c7f77e6ded13c"
            ).to_vec(),
            address: vec![],
            block_index: 0,
            index: 0,
            ordinal: 0,
        };

        assert!(!TokenCreate::match_log(&log), "Log should not match with wrong topic0");
    }

    #[test]
    fn test_sunpump_token_create_wrong_data_length() {
        // Test that match_log rejects a log with incorrect data length
        let log = Log {
            topics: vec![
                hex!("1ff0a01c8968e3551472812164f233abb579247de887db8cbb18281c149bee7a").to_vec(),
            ],
            data: hex!(
                "000000000000000000000000891cdb91d149f23b1a45d9c5ca78a88d0cb44c18"
                "0000000000000000000000000000000000000000000000000000000000000001"
            ).to_vec(), // Only 64 bytes instead of 96
            address: vec![],
            block_index: 0,
            index: 0,
            ordinal: 0,
        };

        assert!(!TokenCreate::match_log(&log), "Log should not match with wrong data length");
    }

    #[test]
    fn test_sunpump_token_create_large_index() {
        // Test with a larger token index value
        let log = Log {
            topics: vec![
                hex!("1ff0a01c8968e3551472812164f233abb579247de887db8cbb18281c149bee7a").to_vec(),
            ],
            data: hex!(
                "00000000000000000000000048c125e0d3c626842bf7180c85e79f97ae524e91"  // tokenAddress
                "00000000000000000000000000000000000000000000000000000000000003e7"  // tokenIndex = 999
                "00000000000000000000000062cd74b7d68d6914ca7e3aa4720d53588f4c18ac"  // creator
            ).to_vec(),
            address: vec![],
            block_index: 0,
            index: 0,
            ordinal: 0,
        };

        match TokenCreate::decode(&log) {
            Ok(event) => {
                assert_eq!(
                    event.token_address,
                    hex!("48c125e0d3c626842bf7180c85e79f97ae524e91"),
                    "Token address should match"
                );
                assert_eq!(
                    event.token_index,
                    BigInt::from(999u64),
                    "Token index should be 999"
                );
                assert_eq!(
                    event.creator,
                    hex!("62cd74b7d68d6914ca7e3aa4720d53588f4c18ac"),
                    "Creator address should match"
                );
            }
            Err(e) => {
                panic!("Error decoding TokenCreate event: {:?}", e);
            }
        }
    }
}
