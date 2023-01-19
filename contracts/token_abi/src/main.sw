library token_abi;

abi Token {
    #[storage(read,write)]
    fn initialize(config:TokenInitializeConfig, owner:Identity);

    #[storage(read)]
    fn total_supply() -> u64;

    #[storage(read)]
    fn decimals() -> u8;

    #[storage(read)]
    fn name() -> str[32];

    #[storage(read)]
    fn symbol() -> str[8];

    #[storage(read,write)]
    fn mint(recipient: Identity, amount: u64);

    // burn users tokens 
    #[storage(read,write)]
    fn burn();

    #[storage(read,write)]
    fn transfer_ownership(new_owner: Identity);

    #[storage(read)]
    fn owner() -> Identity; 
}

// events
pub struct Mint {
    recipient: b256,
    amount: u64,
}

pub struct Burn {
    sender: b256,
    amount: u64,
}

pub struct Ownershiptransferred {
    old_owner: Identity,
    new_owner: Identity,
}


// errors
pub enum Error {
    NotOwner: (),
    ZeroValue: (),
    CannotReinitialize: (),
    WrongAsset: ()
}


pub struct TokenInitializeConfig {
    name: str[32],
    symbol: str[8],
    decimals: u8,
}



