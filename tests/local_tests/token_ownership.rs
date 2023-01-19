use core::fmt;

use fuels::prelude::*;

use crate::utils::local_test_utils::abi_calls::{owner, transfer_ownership};
use crate::utils::local_test_utils::setup_utils::{get_token_instance, setup_token};
use crate::utils::local_test_utils::test_token_mod::Error;
use crate::utils::local_test_utils::{Ownershiptransferred, TestToken};
use fuels::types::errors;

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

    let wallet1_token_instance =
        get_token_instance(token_instance.get_contract_id(), &wallets.wallet1);

    let res = transfer_ownership(
        &wallet1_token_instance,
        Identity::Address(Address::from(wallets.wallet1.address())),
    )
    .await;
    assert!(res.is_err());

    let err = res.unwrap_err();
    if let errors::Error::RevertTransactionError(err_str, _receipt_vec) = &err {
        assert_eq!(err_str, &Error::NotOwner().to_string());
    }
}

#[tokio::test]
async fn owner_mint_tokens() {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
