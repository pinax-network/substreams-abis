#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use substreams_abis::evm::ens::publicresolver::events::AddrChanged;
    use substreams_ethereum::pb::eth::v2::Log;
    use substreams::hex;
    use substreams::scalar::BigInt;

    #[test]
    fn test_ens_resolver() {
        // Example log for AddrChanged event
        let log = Log {
            topics: vec![
                // AddrChanged event signature
                hex!("52d7d861f09ab3d26239d492e8968629f95e9e318cf0b73bfddc441522a15fd2").to_vec(),
                // node (indexed parameter)
                hex!("0000000000000000000000000000000000000000000000000000000000000000").to_vec(),
            ],
            data: hex!("000000000000000000000000b8c2c29ee19d8307cb7255e1cd9cbde883a267d5").to_vec(),
            address: hex!("4976fb03C32e5B8cfe2b6cCB31c09Ba78EBaBa41").to_vec(), // ENS Public Resolver
            block_index: 0,
            index: 0,
            ordinal: 0,
        };

        match AddrChanged::decode(&log) {
            Ok(event) => {
                assert_eq!(event.a, hex!("b8c2c29ee19d8307cb7255e1cd9cbde883a267d5"));
            }
            Err(e) => {
                panic!("Error decoding AddrChanged event: {:?}", e);
            }
        }
    }
}
