#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, PartialEq, TypeAbi, Clone, Copy)]
pub enum Status {
    FundingPeriod,
    Successful,
    Failed,
}

#[elrond_wasm::contract]
pub trait Crowdfunding {
    #[init]
    fn init(&self, target: BigUint, deadline: u64, erc20_contract_address: Address) {
        self.erc20_contract_address().set(&erc20_contract_address);
        self.target().set(&target);
        self.deadline().set(&deadline);
    }

    #[endpoint]
    fn fund(&self, token_amount: BigUint) -> SCResult<AsyncCall<Self::SendApi>> {
        require!(
            self.blockchain().get_block_nonce() <= self.deadline().get(),
            "cannot fund after deadline"
        );

        let caller = self.blockchain().get_caller_managed();
        let erc20_address = self.erc20_contract_address().get();
        let cf_contract_address = self.blockchain().get_sc_address_managed();

        Ok(self
            .erc20_proxy(erc20_address)
            .transfer_from(caller.clone(), cf_contract_address, token_amount.clone())
            .async_call()
            .with_callback(
                self.callbacks()
                    .transfer_from_callback(caller, token_amount),
            ))
    }

    #[view]
    fn status(&self) -> Status {
        if self.blockchain().get_block_nonce() <= self.deadline().get() {
            Status::FundingPeriod
        } else if self
            .blockchain()
            .get_sc_balance(&self.types().token_identifier_egld(), 0)
            >= self.target().get()
        {
            Status::Successful
        } else {
            Status::Failed
        }
    }

    #[endpoint]
    fn claim(&self) -> SCResult<OptionalResult<AsyncCall<Self::SendApi>>> {
        match self.status() {
            Status::FundingPeriod => sc_error!("cannot claim before deadline"),
            Status::Successful => {
                let caller = self.blockchain().get_caller_managed();
                if caller != self.blockchain().get_owner_address_managed() {
                    return sc_error!("only owner can claim successful funding");
                }

                let balance = self.total_balance().get();
                self.total_balance().clear();

                let erc20_address = self.erc20_contract_address().get();
                Ok(OptionalResult::Some(
                    self.erc20_proxy(erc20_address)
                        .transfer(caller, balance)
                        .async_call(),
                ))
            },
            Status::Failed => {
                let caller = self.blockchain().get_caller_managed();
                let deposit = self.deposit(&caller).get();

                if deposit > 0 {
                    self.deposit(&caller).clear();

                    let erc20_address = self.erc20_contract_address().get();
                    Ok(OptionalResult::Some(
                        self.erc20_proxy(erc20_address)
                            .transfer(caller, deposit)
                            .async_call(),
                    ))
                } else {
                    Ok(OptionalResult::None)
                }
            },
        }
    }

    #[callback]
    fn transfer_from_callback(
        &self,
        #[call_result] result: AsyncCallResult<()>,
        cb_sender: ManagedAddress,
        cb_amount: BigUint,
    ) -> OptionalResult<AsyncCall<Self::SendApi>> {
        match result {
            AsyncCallResult::Ok(()) => {
                // transaction started before deadline, ended after -> refund
                if self.blockchain().get_block_nonce() > self.deadline().get() {
                    let erc20_address = self.erc20_contract_address().get();
                    return OptionalResult::Some(
                        self.erc20_proxy(erc20_address)
                            .transfer(cb_sender, cb_amount)
                            .async_call(),
                    );
                }

                self.deposit(&cb_sender)
                    .update(|deposit| *deposit += &cb_amount);
                self.total_balance().update(|balance| *balance += cb_amount);

                OptionalResult::None
            },
            AsyncCallResult::Err(_) => OptionalResult::None,
        }
    }

    // proxy

    #[proxy]
    fn erc20_proxy(&self, to: Address) -> erc20::Proxy<Self::SendApi>;

    // storage

    #[view(get_target)]
    #[storage_mapper("target")]
    fn target(&self) -> SingleValueMapper<Self::Storage, BigUint>;

    #[view(get_deadline)]
    #[storage_mapper("deadline")]
    fn deadline(&self) -> SingleValueMapper<Self::Storage, u64>;

    #[view(get_deposit)]
    #[storage_mapper("deposit")]
    fn deposit(&self, donor: &ManagedAddress) -> SingleValueMapper<Self::Storage, BigUint>;

    #[view(get_erc20_contract_address)]
    #[storage_mapper("erc20_contract_address")]
    fn erc20_contract_address(&self) -> SingleValueMapper<Self::Storage, Address>;

    #[view(get_total_balance)]
    #[storage_mapper("erc20_balance")]
    fn total_balance(&self) -> SingleValueMapper<Self::Storage, BigUint>;
}
