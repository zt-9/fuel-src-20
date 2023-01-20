contract;

use token_abi::*;

use std::{
    auth::{
        msg_sender,
    },
    call_frames::{
        contract_id,
        msg_asset_id,
    },
    constants::{
        BASE_ASSET_ID,
        ZERO_B256,
    },
    context::{
        msg_amount,
    },
    logging::{
        log,
    },
    token::{
        burn,
        mint_to,
    },
};

pub trait Into<T> {
    fn into(self) -> T;
}

impl Into<b256> for Identity {
    fn into(self) -> b256 {
        match self {
            Identity::Address(identity) => identity.value,
            Identity::ContractId(identity) => identity.value,
        }
    }
}

storage {
    total_supply: u64 = 0,
    owner: Identity = Identity::Address(Address {
        value: ZERO_B256,
    }),
    config: TokenInitializeConfig = TokenInitializeConfig {
        name: "                                ",
        symbol: "        ",
        decimals: 0u8,
    },
}

impl Token for Contract {
    #[storage(read, write)]
    fn initialize(config: TokenInitializeConfig, owner: Identity) {
        require(storage.owner.into() == ZERO_B256, Error::CannotReinitialize);
        _transfer_ownership(owner);
        storage.config = config;
        log(config);
    }

    #[storage(read)]
    fn total_supply() -> u64 {
        storage.total_supply
    }

    #[storage(read)]
    fn decimals() -> u8 {
        storage.config.decimals
    }

    #[storage(read)]
    fn name() -> str[32] {
        storage.config.name
    }

    #[storage(read)]
    fn symbol() -> str[8] {
        storage.config.symbol
    }

    #[storage(read, write)]
    fn mint(recipient: Identity, amount: u64) {
        _validate_owner();
        storage.total_supply += amount;
        mint_to(amount, recipient); // TODO: revert bc of the transfer_to() inside this method. mint along works well
        log(Mint {
            recipient: recipient.into(),
            amount: amount,
        })
    }

    #[storage(read, write)]
    fn burn() {
        require(msg_asset_id() == contract_id(), Error::WrongAsset);
        let sender = msg_sender().unwrap().into();
        let amount = msg_amount();
        require(msg_amount() != 0, Error::ZeroValue);

        storage.total_supply -= amount;
        burn(msg_amount());
        log(Burn { sender, amount })
    }

    #[storage(read, write)]
    fn transfer_ownership(new_owner: Identity) {
        _validate_owner();
        _transfer_ownership(new_owner);
    }

    #[storage(read)]
    fn owner() -> Identity {
        storage.owner
    }
}

#[storage(read)]
fn _validate_owner() {
    let sender = msg_sender().unwrap();
    require(storage.owner == sender, Error::NotOwner);
}

#[storage(read, write)]
fn _transfer_ownership(new_owner: Identity) {
    require(new_owner.into() != ZERO_B256, Error::ZeroValue);
    log(Ownershiptransferred {
        old_owner: storage.owner,
        new_owner: new_owner,
    });
    storage.owner = new_owner;
}
