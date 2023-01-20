use core::fmt;
use std::ops::Add;

use fuels::prelude::*;
use fuels::signers::fuel_crypto::coins_bip32::ecdsa::digest::typenum::Bit;

use crate::utils::local_test_utils::abi_calls::{mint, owner, transfer_ownership};
use crate::utils::local_test_utils::setup_utils::{get_token_instance, setup_token};
use crate::utils::local_test_utils::test_token_mod::Error;
use crate::utils::local_test_utils::{Mint, Ownershiptransferred};
use fuels::types::errors;
use std::convert::Into;

#[tokio::test]
async fn owner_transfer_ownership() {
    let (token_instance, wallets) = setup_token("My Token", "MTK", 18).await;

    let old_owner = Identity::Address(Address::from(wallets.wallet_owner.address()));
    let new_owner = Identity::Address(Address::from(wallets.wallet1.address()));

    let res = transfer_ownership(&token_instance, new_owner.to_owned())
        .await
        .unwrap();

    // owner
    let owner = owner(&token_instance).await.unwrap().value;
    assert_eq!(owner, new_owner);

    // expect to emit log <Ownershiptransferred>
    let log = res.get_logs_with_type::<Ownershiptransferred>().unwrap();
    let expected_log = Ownershiptransferred {
        old_owner,
        new_owner,
    };

    assert_eq!(log, vec![expected_log]);
}

#[tokio::test]
async fn only_owner_can_transfer_ownership() {
    let (token_instance, wallets) = setup_token("My Token", "MTK", 18).await;
    let new_owner = Identity::Address(Address::from(wallets.wallet1.address()));

    let wallet1_token_instance = token_instance.with_wallet(wallets.wallet2).unwrap();

    // perform call with wallet 1
    let res = transfer_ownership(&wallet1_token_instance, new_owner).await;

    assert!(res.is_err());

    let err = res.unwrap_err();
    if let errors::Error::RevertTransactionError(err_str, _) = &err {
        assert_eq!(err_str, &Error::NotOwner().to_string());
    }
}

#[tokio::test]
async fn owner_mint_tokens() {
    let (token_instance, wallets) = setup_token("My Token", "MTK", 18).await;

    // let old_owner = Identity::Address(Address::from(wallets.wallet_owner.address()));
    let recipient = Identity::Address(Address::from(wallets.wallet1.address()));

    let amount = 123232;

    let res = mint(&token_instance, recipient.clone(), amount)
        .await
        .unwrap();

    // should log Mint
    let log = res.get_logs_with_type::<Mint>().unwrap();
    let recipient_bit256: Bits256 = MyIdentity(recipient.clone()).into();
    let expected_log = Mint {
        recipient: recipient_bit256,
        amount,
    };

    assert_eq!(log, vec![expected_log]);
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

struct MyIdentity(Identity); // a tuple struct wrapper
impl Into<Bits256> for MyIdentity {
    fn into(self) -> Bits256 {
        let MyIdentity(identity) = self;
        match identity {
            Identity::Address(value) => {
                let hex_str = value.to_string();
                Bits256::from_hex_str(&hex_str).unwrap()
            }

            Identity::ContractId(value) => {
                let hex_str = value.to_string();
                Bits256::from_hex_str(&hex_str).unwrap()
            }
        }
    }
}
