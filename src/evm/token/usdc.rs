// @generated
// This file was @generated by `substreams-ethereum-abigen`. Do not edit it by hand.

const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
/// Contract's functions.
#[allow(dead_code, unused_imports, unused_variables)]
pub mod functions {
    use super::INTERNAL_ERR;
    #[derive(Debug, Clone, PartialEq)]
    pub struct Admin {}
    impl Admin {
        const METHOD_ID: [u8; 4] = [248u8, 81u8, 164u8, 64u8];
        pub fn decode(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Self, String> {
            Ok(Self {})
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Vec<u8>, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Address], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(
                values
                    .pop()
                    .expect("one output data should have existed")
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            )
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall { to_addr : address, data : self.encode(), }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME, err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for Admin {
        const NAME: &'static str = "admin";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for Admin {
        fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct ChangeAdmin {
        pub new_admin: Vec<u8>,
    }
    impl ChangeAdmin {
        const METHOD_ID: [u8; 4] = [143u8, 40u8, 57u8, 112u8];
        pub fn decode(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                    &[ethabi::ParamType::Address],
                    maybe_data.unwrap(),
                )
                .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                new_admin: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(
                &[ethabi::Token::Address(ethabi::Address::from_slice(&self.new_admin))],
            );
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }
    impl substreams_ethereum::Function for ChangeAdmin {
        const NAME: &'static str = "changeAdmin";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Implementation {}
    impl Implementation {
        const METHOD_ID: [u8; 4] = [92u8, 96u8, 218u8, 27u8];
        pub fn decode(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Self, String> {
            Ok(Self {})
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Vec<u8>, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Address], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(
                values
                    .pop()
                    .expect("one output data should have existed")
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            )
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall { to_addr : address, data : self.encode(), }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME, err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for Implementation {
        const NAME: &'static str = "implementation";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for Implementation {
        fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct UpgradeTo {
        pub new_implementation: Vec<u8>,
    }
    impl UpgradeTo {
        const METHOD_ID: [u8; 4] = [54u8, 89u8, 207u8, 230u8];
        pub fn decode(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                    &[ethabi::ParamType::Address],
                    maybe_data.unwrap(),
                )
                .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                new_implementation: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(
                &[
                    ethabi::Token::Address(
                        ethabi::Address::from_slice(&self.new_implementation),
                    ),
                ],
            );
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }
    impl substreams_ethereum::Function for UpgradeTo {
        const NAME: &'static str = "upgradeTo";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct UpgradeToAndCall {
        pub new_implementation: Vec<u8>,
        pub data: Vec<u8>,
    }
    impl UpgradeToAndCall {
        const METHOD_ID: [u8; 4] = [79u8, 30u8, 242u8, 134u8];
        pub fn decode(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                    &[ethabi::ParamType::Address, ethabi::ParamType::Bytes],
                    maybe_data.unwrap(),
                )
                .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                new_implementation: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                data: values.pop().expect(INTERNAL_ERR).into_bytes().expect(INTERNAL_ERR),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(
                &[
                    ethabi::Token::Address(
                        ethabi::Address::from_slice(&self.new_implementation),
                    ),
                    ethabi::Token::Bytes(self.data.clone()),
                ],
            );
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }
    impl substreams_ethereum::Function for UpgradeToAndCall {
        const NAME: &'static str = "upgradeToAndCall";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
}
/// Contract's events.
#[allow(dead_code, unused_imports, unused_variables)]
pub mod events {
    use super::INTERNAL_ERR;
    #[derive(Debug, Clone, PartialEq)]
    pub struct AdminChanged {
        pub previous_admin: Vec<u8>,
        pub new_admin: Vec<u8>,
    }
    impl AdminChanged {
        const TOPIC_ID: [u8; 32] = [
            126u8,
            100u8,
            77u8,
            121u8,
            66u8,
            47u8,
            23u8,
            192u8,
            30u8,
            72u8,
            148u8,
            181u8,
            244u8,
            245u8,
            136u8,
            211u8,
            49u8,
            235u8,
            250u8,
            40u8,
            101u8,
            61u8,
            66u8,
            174u8,
            131u8,
            45u8,
            197u8,
            158u8,
            56u8,
            201u8,
            121u8,
            143u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 1usize {
                return false;
            }
            if log.data.len() != 64usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() as &[u8]
                == Self::TOPIC_ID;
        }
        pub fn decode(
            log: &substreams_ethereum::pb::eth::v2::Log,
        ) -> Result<Self, String> {
            let mut values = ethabi::decode(
                    &[ethabi::ParamType::Address, ethabi::ParamType::Address],
                    log.data.as_ref(),
                )
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                previous_admin: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                new_admin: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
    }
    impl substreams_ethereum::Event for AdminChanged {
        const NAME: &'static str = "AdminChanged";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Upgraded {
        pub implementation: Vec<u8>,
    }
    impl Upgraded {
        const TOPIC_ID: [u8; 32] = [
            188u8,
            124u8,
            215u8,
            90u8,
            32u8,
            238u8,
            39u8,
            253u8,
            154u8,
            222u8,
            186u8,
            179u8,
            32u8,
            65u8,
            247u8,
            85u8,
            33u8,
            77u8,
            188u8,
            107u8,
            255u8,
            169u8,
            12u8,
            192u8,
            34u8,
            91u8,
            57u8,
            218u8,
            46u8,
            92u8,
            45u8,
            59u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 1usize {
                return false;
            }
            if log.data.len() != 32usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() as &[u8]
                == Self::TOPIC_ID;
        }
        pub fn decode(
            log: &substreams_ethereum::pb::eth::v2::Log,
        ) -> Result<Self, String> {
            let mut values = ethabi::decode(
                    &[ethabi::ParamType::Address],
                    log.data.as_ref(),
                )
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                implementation: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
    }
    impl substreams_ethereum::Event for Upgraded {
        const NAME: &'static str = "Upgraded";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
}