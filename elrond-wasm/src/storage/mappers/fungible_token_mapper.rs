use elrond_codec::{EncodeErrorHandler, TopEncodeMulti, TopEncodeMultiOutput};

use super::{
    token_mapper::{StorageTokenWrapper, TOKEN_ID_ALREADY_SET_ERR_MSG},
    StorageMapper,
};
use crate::{
    api::{BlockchainApiImpl, CallTypeApi, ErrorApiImpl, StorageMapperApi},
    contract_base::SendWrapper,
    esdt::{ESDTSystemSmartContractProxy, FungibleTokenProperties},
    storage::StorageKey,
    types::{
        BigUint, CallbackClosure, EsdtTokenPayment, ManagedAddress, ManagedBuffer, ManagedRef,
        ManagedType, TokenIdentifier,
    },
};

pub(crate) const DEFAULT_ISSUE_CALLBACK_NAME: &[u8] = b"default_issue_cb";
pub(crate) const DEFAULT_ISSUE_WITH_INIT_SUPPLY_CALLBACK_NAME: &[u8] =
    b"default_issue_init_supply_cb";

pub struct FungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    key: StorageKey<SA>,
}

impl<SA> StorageMapper<SA> for FungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        Self { key: base_key }
    }
}

impl<SA> StorageTokenWrapper<SA> for FungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    fn get_storage_key(&self) -> ManagedRef<SA, StorageKey<SA>> {
        self.key.as_ref()
    }
}

impl<SA> FungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    /// Important: If you use custom callback, remember to save the token ID in the callback!
    /// If you want to use default callbacks, import the default_issue_callbacks::DefaultIssueCallbacksModule from elrond-wasm-modules
    /// and pass None for the opt_callback argument
    pub fn issue(
        &self,
        issue_cost: BigUint<SA>,
        token_display_name: ManagedBuffer<SA>,
        token_ticker: ManagedBuffer<SA>,
        initial_supply: BigUint<SA>,
        num_decimals: usize,
        opt_callback: Option<CallbackClosure<SA>>,
    ) -> ! {
        if !self.is_empty() {
            SA::error_api_impl().signal_error(TOKEN_ID_ALREADY_SET_ERR_MSG);
        }

        let system_sc_proxy = ESDTSystemSmartContractProxy::<SA>::new_proxy_obj();
        let callback = match opt_callback {
            Some(cb) => cb,
            None => self.default_callback_closure_obj(&initial_supply),
        };

        system_sc_proxy
            .issue_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                &initial_supply,
                FungibleTokenProperties {
                    num_decimals,
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_mint: false,
                    can_burn: false,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(callback)
            .call_and_exit();
    }

    fn default_callback_closure_obj(&self, initial_supply: &BigUint<SA>) -> CallbackClosure<SA> {
        let initial_caller =
            ManagedAddress::<SA>::from_raw_handle(SA::blockchain_api_impl().get_caller_handle());
        let cb_name = if initial_supply > &0 {
            DEFAULT_ISSUE_WITH_INIT_SUPPLY_CALLBACK_NAME
        } else {
            DEFAULT_ISSUE_CALLBACK_NAME
        };

        let mut cb_closure = CallbackClosure::new(cb_name.into());
        cb_closure.push_endpoint_arg(&initial_caller);
        cb_closure.push_endpoint_arg(&self.key.buffer);

        cb_closure
    }

    pub fn mint(&self, amount: BigUint<SA>) -> EsdtTokenPayment<SA> {
        let send_wrapper = SendWrapper::<SA>::new();
        let token_id = self.get_token_id();

        send_wrapper.esdt_local_mint(&token_id, 0, &amount);

        EsdtTokenPayment::new(token_id, 0, amount)
    }

    pub fn mint_and_send(
        &self,
        to: &ManagedAddress<SA>,
        amount: BigUint<SA>,
    ) -> EsdtTokenPayment<SA> {
        let payment = self.mint(amount);
        self.send_payment(to, &payment);

        payment
    }

    pub fn burn(&self, amount: &BigUint<SA>) {
        let send_wrapper = SendWrapper::<SA>::new();
        let token_id = self.get_token_id();

        send_wrapper.esdt_local_burn(&token_id, 0, &amount);
    }

    fn send_payment(&self, to: &ManagedAddress<SA>, payment: &EsdtTokenPayment<SA>) {
        let send_wrapper = SendWrapper::<SA>::new();
        send_wrapper.direct(to, &payment.token_identifier, 0, &payment.amount, &[]);
    }
}

impl<SA> TopEncodeMulti for FungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    type DecodeAs = TokenIdentifier<SA>;

    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        if self.is_empty() {
            output.push_single_value(&ManagedBuffer::<SA>::new(), h)
        } else {
            output.push_single_value(&self.get_token_id(), h)
        }
    }
}
