mod adder_interact_cli;
mod adder_interact_config;
mod adder_interact_state;

use adder::ProxyTrait;
use adder_interact_config::Config;
use adder_interact_state::State;
use clap::Parser;
use multiversx_sc_snippets::{
    env_logger,
    multiversx_sc::{
        storage::mappers::SingleValue,
        types::{Address, CodeMetadata},
    },
    multiversx_sc_scenario::{
        bech32,
        num_bigint::BigUint,
        scenario_format::interpret_trait::InterpreterContext,
        scenario_model::{IntoBlockchainCall, TransferStep, TxExpect},
        test_wallets, ContractInfo, DebugApi,
    },
    tokio, Interactor,
};

#[tokio::main]
async fn main() {
    DebugApi::dummy();
    env_logger::init();

    let mut adder_interact = AdderInteract::init().await;

    let cli = adder_interact_cli::InteractCli::parse();
    match &cli.command {
        Some(adder_interact_cli::InteractCliCommand::Add(args)) => {
            adder_interact.add(args.value).await;
        },
        Some(adder_interact_cli::InteractCliCommand::Deploy) => {
            adder_interact.deploy().await;
        },
        Some(adder_interact_cli::InteractCliCommand::Feed) => {
            adder_interact.feed_contract_egld().await;
        },
        Some(adder_interact_cli::InteractCliCommand::Sum) => {
            adder_interact.print_sum().await;
        },
        None => {},
    }
}

#[allow(unused)]
struct AdderInteract {
    interactor: Interactor,
    wallet_address: Address,
    state: State,
}

impl AdderInteract {
    async fn init() -> Self {
        let config = Config::load_config();
        let mut interactor = Interactor::new(config.gateway()).await;
        let wallet_address = interactor.register_wallet(test_wallets::mike());

        Self {
            interactor,
            wallet_address,
            state: State::load_state(),
        }
    }

    async fn deploy(&mut self) {
        let mut typed_sc_deploy = self
            .state
            .default_adder()
            .init(BigUint::from(0u64))
            .into_blockchain_call()
            .from(&self.wallet_address)
            .code_metadata(CodeMetadata::all())
            .contract_code("file:../output/adder.wasm", &InterpreterContext::default())
            .gas_limit("70,000,000")
            .expect(TxExpect::ok());

        self.interactor.sc_deploy(&mut typed_sc_deploy).await;

        let result = typed_sc_deploy.response().new_deployed_address();
        if result.is_err() {
            println!("deploy failed: {}", result.err().unwrap());
            return;
        }

        let new_address_bech32 = bech32::encode(&result.unwrap());
        println!("new address: {new_address_bech32}");

        let new_address_expr = format!("bech32:{new_address_bech32}");
        self.state.set_adder_address(&new_address_expr);
    }

    async fn feed_contract_egld(&mut self) {
        let _ = self
            .interactor
            .transfer(
                TransferStep::new()
                    .from(&self.wallet_address)
                    .to(self.state.adder())
                    .egld_value("0,050000000000000000"),
            )
            .await;
    }

    async fn add(&mut self, value: u64) {
        let mut typed_sc_call = self
            .state
            .adder()
            .add(BigUint::from(value))
            .into_blockchain_call()
            .from(&self.wallet_address)
            .gas_limit("70,000,000")
            .expect(TxExpect::ok());

        self.interactor.sc_call(&mut typed_sc_call).await;

        let result = typed_sc_call.response().handle_signal_error_event();
        if result.is_err() {
            println!("performing add failed with: {}", result.err().unwrap());
            return;
        }

        println!("successfully performed add");
    }

    async fn print_sum(&mut self) {
        let sum: SingleValue<BigUint> = self.interactor.vm_query(self.state.adder().sum()).await;
        println!("sum: {}", sum.into());
    }
}
