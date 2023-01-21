use fuels::prelude::*;
use fuels::programs::call_response::FuelCallResponse;
use fuels::signers::WalletUnlocked;
use fuels::types::{Identity, SizedAsciiString};

abigen!(Contract(
    name = "TestToken",
    abi = "contracts/token_contract/out/debug/token_contract-abi.json"
));

pub struct WalletSetup {
    pub wallet_owner: WalletUnlocked,
    pub wallet1: WalletUnlocked,
    pub wallet2: WalletUnlocked,
}

pub mod abi_calls {

    use fuels::types::errors::Error;

    use super::*;

    pub async fn initialize(
        contract: &TestToken,
        name: &str,
        symbol: &str,
        decimals: u8,
        owner: Identity,
    ) -> Result<FuelCallResponse<()>, Error> {
        let mut name = name.to_string();
        let mut symbol = symbol.to_string();
        name.push_str(" ".repeat(32 - name.len()).as_str());
        symbol.push_str(" ".repeat(8 - symbol.len()).as_str());

        contract
            .methods()
            .initialize(
                TokenInitializeConfig {
                    name: SizedAsciiString::<32>::new(name).unwrap(),
                    symbol: SizedAsciiString::<8>::new(symbol).unwrap(),
                    decimals,
                },
                owner,
            )
            .call()
            .await
    }

    pub async fn total_supply(contract: &TestToken) -> Result<FuelCallResponse<u64>, Error> {
        contract.methods().total_supply().call().await
    }

    pub async fn decimals(contract: &TestToken) -> Result<FuelCallResponse<u8>, Error> {
        contract.methods().decimals().call().await
    }

    pub async fn name(
        contract: &TestToken,
    ) -> Result<FuelCallResponse<SizedAsciiString<32>>, Error> {
        contract.methods().name().call().await
    }

    pub async fn symbol(
        contract: &TestToken,
    ) -> Result<FuelCallResponse<SizedAsciiString<8>>, Error> {
        contract.methods().symbol().call().await
    }

    pub async fn owner(contract: &TestToken) -> Result<FuelCallResponse<Identity>, Error> {
        contract.methods().owner().call().await
    }

    pub async fn mint(
        contract: &TestToken,
        receipient: Identity,
        amount: u64,
    ) -> Result<FuelCallResponse<()>, Error> {
        contract
            .methods()
            .mint(receipient, amount)
            .append_variable_outputs(1)
            .call()
            .await
    }

    pub async fn burn(contract: &TestToken, amount: u64) -> Result<FuelCallResponse<()>, Error> {
        contract
            .methods()
            .burn()
            .call_params(CallParameters::new(Some(amount), None, None))
            .call()
            .await
    }

    pub async fn transfer_ownership(
        contract: &TestToken,
        owner: Identity,
    ) -> Result<FuelCallResponse<()>, Error> {
        contract.methods().transfer_ownership(owner).call().await
    }
}

pub mod setup_utils {
    use super::*;
    pub async fn setup_wallets() -> WalletSetup {
        let initial_amount: u64 = 10 ^ 18;
        let num_wallets: u64 = 3;
        let num_coins = 1;

        let config = WalletsConfig::new(Some(num_wallets), Some(num_coins), Some(initial_amount));
        let wallets = launch_custom_provider_and_get_wallets(config, None, None).await;
        let wallet_owner = wallets.get(0).unwrap().clone();
        let wallet1 = wallets.get(1).unwrap().clone();
        let wallet2 = wallets.get(2).unwrap().clone();

        WalletSetup {
            wallet_owner,
            wallet1,
            wallet2,
        }
    }

    pub async fn setup_token_contract(wallet_owner: &WalletUnlocked) -> TestToken {
        let token_contract_id = Contract::deploy(
            "contracts/token_contract/out/debug/token_contract.bin",
            wallet_owner,
            TxParameters::default(),
            StorageConfiguration::with_storage_path(Some(
                "contracts/token_contract/out/debug/token_contract-storage_slots.json".to_string(),
            )),
        )
        .await
        .unwrap();

        get_token_instance(&token_contract_id, wallet_owner)
    }

    pub fn get_token_instance(
        token_contract_id: &Bech32ContractId,
        wallet: &WalletUnlocked,
    ) -> TestToken {
        TestToken::new(token_contract_id.clone(), wallet.clone())
    }

    pub async fn setup() -> (TestToken, WalletSetup) {
        let wallets = setup_wallets().await;
        let test_token = setup_token_contract(&wallets.wallet_owner).await;
        (test_token, wallets)
    }

    // return a owner_token_instance
    pub async fn setup_token(
        token_name: &str,
        token_symbol: &str,
        decimals: u8,
    ) -> (TestToken, WalletSetup) {
        let (token_instance, wallets) = setup().await;
        // println!(
        //     " 🪙  Token contract id: {}",
        //     token_instance.get_contract_id()
        // );

        // println!(" 👮 Wallet owner     : {}", wallets.wallet_owner.address());

        abi_calls::initialize(
            &token_instance,
            token_name,
            token_symbol,
            decimals,
            Identity::Address(Address::from(wallets.wallet_owner.address())),
        )
        .await
        .unwrap();

        (token_instance, wallets)
    }
}
