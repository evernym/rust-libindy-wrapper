#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde;
extern crate byteorder;
extern crate rust_libindy_wrapper as indy;
#[allow(unused_variables)]
#[allow(unused_macros)]
#[allow(dead_code)]
#[macro_use]
pub mod utils;

use indy::did::Did;
use indy::ErrorCode;
use indy::ledger::Ledger;
use indy::pool::Pool;
use std::sync::mpsc::channel;
use std::time::Duration;
use utils::constants::{INVALID_TIMEOUT, PROTOCOL_VERSION, VALID_TIMEOUT};
use utils::setup::{Setup, SetupConfig};
use utils::wallet::Wallet;

const REQUEST_JSON: &str = r#"{
                              "reqId":1496822211362017764,
                              "identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
                              "operation":{
                                   "type":"1",
                                   "dest":"VsKV7grR1BUE29mG2Fm2kX",
                                   "verkey":"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
                                   },
                              "protocolVersion":2
                          }"#;
#[cfg(test)]
mod test_sign_and_submit_request {

    use super::*;


    // the purpose of this test is to show a crash.  This bug needs to be researched as a possible bug
    // in libindy
    #[test]
    #[ignore]
    pub fn sign_and_submit_request_crashes() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let result = indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet.handle, &did, "{}");

        let mut response : String = "".to_string();

        match result {
            Ok(return_response) => { response = return_response; },
            Err(ec) => { assert!(false, "sign_and_submit_request_success got error code {:?}", ec); },
        }

        Pool::close(pool_handle).unwrap();

        assert!(false, "response {:?}", response);
    }

    #[test]
    pub fn sign_and_submit_request_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let result = Ledger::sign_and_submit_request(pool_handle, wallet.handle, &did, REQUEST_JSON);

        Pool::close(pool_handle).unwrap();

        match result {
            Ok(_) => { },
            Err(ec) => { assert!(false, "sign_and_submit_request_success got error code {:?}", ec); },
        }


        /*
         * The format of SignAndSubmitRequestAsync response is like this.
         *
            {"result":{
                "reqSignature":{
                    "type":"ED25519",
                    "values":[{"value":"7kDrVBrmrKAvSs1QoQWYq6F774ZN3bRXx5e3aaUFiNvmh4F1yNqQw1951Az35nfrnGjZ99vtCmSFXZ5GqS1zLiG","from":"V4SGRU86Z58d6TV7PBUe6f"}]
                },
                "txnMetadata":{
                    "txnTime":1536876204,
                    "seqNo":36,
                    "txnId":"5d38ac6a242239c97ee28884c2b5cadec62248b2256bce51afd814c7847a853e"
                },
                "ver":"1",
                "auditPath":["DATtzSu9AMrArv8C2oribQh4wJ6TaD2K9o76t7EL2N7G","AbGuM7s9MudnT8M2eZe1yaG2EGUGxggMXSSbXCm4DFDx","3fjMoUdsbNrRfG5ZneHaQuX994oA4Z2pYPZtRRPmkngw"],
                "rootHash":"A9LirjLuoBT59JJTJYvUgfQyEJA32Wb7njrbD9XqT2wc",
                "txn":{
                    "data":{
                        "dest":"KQRpY4EmSG4MwH7md8gMoN","verkey":"B2nW4JfqZ2omHksoCmwD8zXXmtBsvbQk6WVSboazd8QB"
                    },
                    "protocolVersion":2,
                    "type":"1",
                    "metadata":{
                        "digest":"14594e0b31f751faf72d4bf4abdc6f54af34dab855fe1a0c67fe651b47bb93b5","reqId":1536876205519496000,"from":"V4SGRU86Z58d6TV7PBUe6f"
                    }
                }
            },
            "op":"REPLY"}
        */
    }

    #[test]
    pub fn sign_and_submit_request_async_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };

        Ledger::sign_and_submit_request_async(pool_handle, wallet.handle, &did, REQUEST_JSON, cb);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();

        Pool::close(pool_handle).unwrap();

        assert_eq!(ec, ErrorCode::Success);
    }

    #[test]
    pub fn sign_and_submit_request_timeout_success() {

        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let result = Ledger::sign_and_submit_request_timeout(pool_handle, wallet.handle, &did, REQUEST_JSON, VALID_TIMEOUT);
        Pool::close(pool_handle).unwrap();

        match result {
            Ok(_) => {  },
            Err(ec) => { assert!(false, "sign_and_submit_request_timeout_success got error code {:?}", ec); },
        }


    }

    #[test]
    pub fn sign_and_submit_request_timeout_times_out() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();;
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let result = Ledger::sign_and_submit_request_timeout(pool_handle, wallet.handle, &did, REQUEST_JSON, INVALID_TIMEOUT);
        Pool::close(pool_handle).unwrap();

        match result {
            Ok(_) => {
                assert!(false, "sign_and_submit_request_timeout DID NOT time out");
            },
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "sign_and_submit_request_timeout error code didn't match expected => {:?}", ec);
            },
        }
    }

}

#[cfg(test)]
mod test_submit_request {
    use super::*;

    #[test]
    pub fn submit_request_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();
        let (_, _) = Did::new(wallet.handle, "{}").unwrap();

        let submit_request_result = Ledger::submit_request(pool_handle, REQUEST_JSON);

        Pool::close(pool_handle).unwrap();

        match submit_request_result {
            Ok(submit_request_response) => {
                // return is REQNACK client request invalid: MissingSignature()....this is ok.  we wanted to make sure the function works
                // and getting that response back indicates success
                assert!(submit_request_response.contains("REQNACK"), "submit_request did not return REQNACK => {:?}", submit_request_response);
                assert!(submit_request_response.contains("MissingSignature"), "submit_request did not return MissingSignature => {:?}", submit_request_response);
            },
            Err(ec) => {
                assert!(false, "submit_request failed with {:?}", ec);
            }
        }

    }

    #[test]
    pub fn submit_request_async_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();
        let (_, _) = Did::new(wallet.handle, "{}").unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };

        indy::ledger::Ledger::submit_request_async(pool_handle, REQUEST_JSON, cb);

        let (ec, submit_request_response) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();

        Pool::close(pool_handle).unwrap();

        assert_eq!(ec, ErrorCode::Success, "submit_request did not return ErrorCode::Success => {:?}", ec);

        // return is REQNACK client request invalid: MissingSignature()....this is ok.  we wanted to make sure the function works
        // and getting that response back indicates success
        assert!(submit_request_response.contains("REQNACK"), "submit_request did not return REQNACK => {:?}", submit_request_response);
        assert!(submit_request_response.contains("MissingSignature"), "submit_request did not return MissingSignature => {:?}", submit_request_response);
    }

    #[test]
    pub fn submit_request_timeout_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();
        let (_, _) = Did::new(wallet.handle, "{}").unwrap();

        let submit_request_result = indy::ledger::Ledger::submit_request_timeout(pool_handle, REQUEST_JSON, VALID_TIMEOUT);

        Pool::close(pool_handle).unwrap();

        match submit_request_result {
            Ok(submit_request_response) => {
                // return is REQNACK client request invalid: MissingSignature()....this is ok.  we wanted to make sure the function works
                // and getting that response back indicates success
                assert!(submit_request_response.contains("REQNACK"), "submit_request did not return REQNACK => {:?}", submit_request_response);
                assert!(submit_request_response.contains("MissingSignature"), "submit_request did not return MissingSignature => {:?}", submit_request_response);
            },
            Err(ec) => {
                assert!(false, "submit_request failed with {:?}", ec);
            }
        }
    }

    #[test]
    pub fn submit_request_timeout_times_out() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = utils::wallet::Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();
        let (_, _) = Did::new(wallet.handle, "{}").unwrap();

        let submit_request_result = indy::ledger::Ledger::submit_request_timeout(pool_handle, REQUEST_JSON, INVALID_TIMEOUT);

        Pool::close(pool_handle).unwrap();

        match submit_request_result {
            Ok(_) => {
                assert!(false, "submit_request_timeout DID NOT time out");
            },
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "submit_request_timeout error code didn't match expected => {:?}", ec);
            },
        }
    }

}

#[cfg(test)]
mod test_submit_action {
    use super::*;

    const NODES : &str = "[\"Node1\", \"Node2\"]";

    // This test needs to be researched as a possible bug in libindy.  No errors are returned, it hangs forever,
    // ignoring the timeout
    #[test]
    pub fn submit_action_this_hangs_indefinitely() {

        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();
        let signed_request = Ledger::sign_request(wallet.handle, &did, &validator_request).unwrap();

        let result = Ledger::submit_action(pool_handle, &signed_request, "[]", 5);

        Pool::close(pool_handle).unwrap();

        match result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "submit_action_success failed with {:?} extra {:?}", ec, signed_request);
            }
        }
    }

    #[test]
    pub fn submit_action_success() {

        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();
        let signed_request = Ledger::sign_request(wallet.handle, &did, &validator_request).unwrap();

        let result = Ledger::submit_action(pool_handle, &signed_request, NODES, 5);

        Pool::close(pool_handle).unwrap();

        match result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "submit_action_success failed with {:?} extra {:?}", ec, signed_request);
            }
        }
    }

    #[test]
    pub fn submit_action_async_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };

        Ledger::submit_action_async(pool_handle, &validator_request, NODES, 5, cb);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();

        Pool::close(pool_handle).unwrap();

        assert_eq!(ec, ErrorCode::Success, "submit_action_async failed error_code {:?}", ec);
    }

    #[test]
    pub fn submit_action_timeout_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();

        let result = Ledger::submit_action_timeout(pool_handle, &validator_request, NODES, 5, VALID_TIMEOUT);

        Pool::close(pool_handle).unwrap();

        match result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "submit_action_timeout failed with {:?} extra {:?}", ec, validator_request);
            }
        }
    }

    #[test]
    pub fn submit_action_timeout_times_out() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();

        let result = Ledger::submit_action_timeout(pool_handle, &validator_request, NODES, 5, INVALID_TIMEOUT);

        Pool::close(pool_handle).unwrap();

        match result {
            Ok(_) => {
                assert!(false, "submit_action_timeout DID NOT timeout as expected");
            },
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "submit_action_timeout failed with {:?}", ec);
            }
        }
    }
}

#[cfg(test)]
mod test_sign_request {
    use super::*;

    #[test]
    pub fn sign_request_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();
        let signed_request_result = Ledger::sign_request(wallet.handle, &did, &validator_request);

        Pool::close(pool_handle).unwrap();

        match signed_request_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "sign_request returned error {:?}", ec);
            }
        }
    }

    #[test]
    pub fn sign_request_async_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };
        let signed_request_result = Ledger::sign_request_async(wallet.handle, &did, &validator_request, cb);

        assert_eq!(signed_request_result, ErrorCode::Success, "sign_request_async failed error_code {:?}", signed_request_result);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        Pool::close(pool_handle).unwrap();

        assert_eq!(ec, ErrorCode::Success, "sign_request_async returned error_code {:?}", ec);
    }

    #[test]
    pub fn sign_request_timeout_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();
        let signed_request_result = Ledger::sign_request_timeout(wallet.handle, &did, &validator_request, VALID_TIMEOUT);

        Pool::close(pool_handle).unwrap();

        match signed_request_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "sign_request_timeout returned error {:?}", ec);
            }
        }
    }

    #[test]
    pub fn sign_request_timeout_times_out() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();
        let signed_request_result = Ledger::sign_request_timeout(wallet.handle, &did, &validator_request, INVALID_TIMEOUT);

        Pool::close(pool_handle).unwrap();

        match signed_request_result {
            Ok(_) => {
                assert!(false, "sign_request_timeout DID NOT timeout as expected");
            },
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "sign_request_timeout failed with {:?}", ec);
            }
        }
    }
}

#[cfg(test)]
mod test_multi_sign_request {
    use super::*;

    #[test]
    pub fn multi_sign_request_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();
        let signed_request_result = Ledger::multi_sign_request(wallet.handle, &did, &validator_request);

        Pool::close(pool_handle).unwrap();

        match signed_request_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "multi_sign_request returned error {:?}", ec);
            }
        }
    }

    #[test]
    pub fn multi_sign_request_async_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };
        let _ = Ledger::multi_sign_request_async(wallet.handle, &did, &validator_request, cb);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        Pool::close(pool_handle).unwrap();

        assert_eq!(ec, ErrorCode::Success, "sign_request_async failed error_code {:?}", ec);
    }

    #[test]
    pub fn multi_sign_request_timeout_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();
        let signed_request_result = Ledger::multi_sign_request_timeout(wallet.handle, &did, &validator_request, VALID_TIMEOUT);

        Pool::close(pool_handle).unwrap();

        match signed_request_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "multi_sign_request_timeout returned error {:?}", ec);
            }
        }
    }

    #[test]
    pub fn multi_sign_request_timeout_times_out() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = Pool::open_ledger(&setup.pool_name, None).unwrap();

        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let validator_request = Ledger::build_get_validator_info_request(&did).unwrap();
        let signed_request_result = Ledger::multi_sign_request_timeout(wallet.handle, &did, &validator_request, INVALID_TIMEOUT);

        Pool::close(pool_handle).unwrap();

        match signed_request_result {
            Ok(_) => {
                assert!(false, "multi_sign_request_timeout DID NOT timeout as expected");
            },
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "multi_sign_request_timeout failed with {:?}", ec);
            }
        }
    }
}

#[cfg(test)]
mod test_build_get_ddo_request {
    use super::*;

    #[test]
    pub fn build_get_ddo_request_success() {
        assert!(false);
    }

    #[test]
    pub fn build_get_ddo_request_async_success() {
        assert!(false);
    }

    #[test]
    pub fn build_get_ddo_request_timeout_success() {
        assert!(false);
    }

    #[test]
    pub fn build_get_ddo_request_timeout_times_out() {
        assert!(false);
    }
}

#[cfg(test)]
mod test_build_nym_request {
    use super::*;

    use utils::did::NymRole;

    #[test]
    pub fn build_nym_request_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();
        let (trustee_did, _) = Did::new(wallet.handle, "{}").unwrap();

        let nym_result = Ledger::build_nym_request(&trustee_did, &did, Some(&verkey), None, NymRole::Trustee.prepare());

        match nym_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_nym_request returned error_code {:?}", ec);
            }
        }

    }

    #[test]
    pub fn build_nym_request_with_no_verkey_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let (trustee_did, _) = Did::new(wallet.handle, "{}").unwrap();

        let nym_result = Ledger::build_nym_request(&trustee_did, &did, None, None, NymRole::Trustee.prepare());

        match nym_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_nym_request returned error_code {:?}", ec);
            }
        }

    }

    #[test]
    pub fn build_nym_request_with_data_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();
        let (trustee_did, _) = Did::new(wallet.handle, "{}").unwrap();

        let nym_result = Ledger::build_nym_request(&trustee_did, &did, None, Some("some_data"), NymRole::Trustee.prepare());

        match nym_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_nym_request returned error_code {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_nym_request_async_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();
        let (trustee_did, _) = Did::new(wallet.handle, "{}").unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };
        let async_error_code = Ledger::build_nym_request_async(&trustee_did, &did, Some(&verkey), None, NymRole::Trustee.prepare(), cb);

        assert_eq!(async_error_code, ErrorCode::Success, "build_nym_request_async failed error_code {:?}", async_error_code);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();

        assert_eq!(ec, ErrorCode::Success, "build_nym_request_async returned error_code {:?}", ec);

    }

    #[test]
    pub fn build_nym_request_timeout_success() {

        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();
        let (trustee_did, _) = Did::new(wallet.handle, "{}").unwrap();

        let nym_result = Ledger::build_nym_request_timeout(&trustee_did, &did, Some(&verkey), None, NymRole::Trustee.prepare(), VALID_TIMEOUT);

        match nym_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_nym_request_timeout returned error_code {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_nym_request_timeout_times_out() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();
        let (trustee_did, _) = Did::new(wallet.handle, "{}").unwrap();

        let nym_result = Ledger::build_nym_request_timeout(&trustee_did, &did, Some(&verkey), None, NymRole::Trustee.prepare(), INVALID_TIMEOUT);

        match nym_result {
            Ok(_) => {
                assert!(false, "build_nym_request_timeout did not time out as expected");
            },
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "build_nym_request_timeout returned error_code {:?}", ec);
            }
        }
    }
}

#[cfg(test)]
mod test_build_get_nym_request {
    use super::*;

    #[test]
    pub fn build_get_nym_request_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let submitter_wallet = Wallet::new();
        let wallet = Wallet::new();
        let (submitter_did, _) = Did::new(submitter_wallet.handle, "{}").unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let get_result = Ledger::build_get_nym_request(Some(&submitter_did), &did);

        match get_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_get_nym_request returned error_code {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_get_nym_request_no_submitter_did_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let wallet = Wallet::new();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let get_result = Ledger::build_get_nym_request(None, &did);

        match get_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_get_nym_request returned error_code {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_get_nym_request_async_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let submitter_wallet = Wallet::new();
        let wallet = Wallet::new();
        let (submitter_did, _) = Did::new(submitter_wallet.handle, "{}").unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let (sender, receiver) = channel();
        let cb = move |ec, stuff| {
            sender.send((ec, stuff)).unwrap();
        };

        let get_async_result = Ledger::build_get_nym_request_async(Some(&submitter_did), &did, cb);

        assert_eq!(get_async_result, ErrorCode::Success, "build_get_nym_request_async failed {:?}", get_async_result);

        let (ec, _) = receiver.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(ec, ErrorCode::Success, "build_get_nym_request_async returned error_code {:?}", ec);
    }

    #[test]
    pub fn build_get_nym_request_timeout_success() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let submitter_wallet = Wallet::new();
        let wallet = Wallet::new();
        let (submitter_did, _) = Did::new(submitter_wallet.handle, "{}").unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let get_result = Ledger::build_get_nym_request_timeout(Some(&submitter_did), &did, VALID_TIMEOUT);

        match get_result {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "build_get_nym_request_timeout returned error_code {:?}", ec);
            }
        }
    }

    #[test]
    pub fn build_get_nym_request_timeout_times_out() {
        Pool::set_protocol_version(PROTOCOL_VERSION as usize).unwrap();

        let submitter_wallet = Wallet::new();
        let wallet = Wallet::new();
        let (submitter_did, _) = Did::new(submitter_wallet.handle, "{}").unwrap();
        let (did, _) = Did::new(wallet.handle, "{}").unwrap();

        let get_result = Ledger::build_get_nym_request_timeout(Some(&submitter_did), &did, INVALID_TIMEOUT);

        match get_result {
            Ok(_) => {
                assert!(false, "build_get_nym_request_timeout DID NOT time out as expected");
            },
            Err(ec) => {
                assert_eq!(ec, ErrorCode::CommonIOError, "build_get_nym_request_timeout returned error_code {:?}", ec);
            }
        }
    }
}

#[cfg(test)]
mod test_build_get_txn_request {

}

#[cfg(test)]
mod test_build_attrib_request {

}

#[cfg(test)]
mod test_build_get_attrib_request {

}

#[cfg(test)]
mod test_build_schema_request {

}

#[cfg(test)]
mod test_build_get_schema_request {

}

#[cfg(test)]
mod test_build_cred_def_request {

}

#[cfg(test)]
mod test_parse_get_schema_response {

}

#[cfg(test)]
mod test_build_get_cred_def_request {

}

#[cfg(test)]
mod test_build_node_request {

}

#[cfg(test)]
mod test_build_get_validator_info_request {

}

#[cfg(test)]
mod test_build_pool_config_request {

}

#[cfg(test)]
mod test_build_pool_restart_request {

}

#[cfg(test)]
mod test_build_pool_upgrade_request {

}

#[cfg(test)]
mod test_build_revoc_reg_def_request {

}

#[cfg(test)]
mod test_build_get_revoc_reg_def_request {

}

#[cfg(test)]
mod test_parse_get_revoc_reg_def_response {

}

#[cfg(test)]
mod test_build_revoc_reg_entry_request {
}

#[cfg(test)]
mod test_build_get_revoc_reg_request {

}

#[cfg(test)]
mod test_parse_get_revoc_reg_response {

}

#[cfg(test)]
mod test_build_get_revoc_reg_delta_request {

}

#[cfg(test)]
mod test_parse_get_revoc_reg_delta_response {

}

#[cfg(test)]
mod test_register_transaction_parser_for_sp {

}