use elrond_wasm::api::ESDT_TRANSFER_FUNC_NAME;

use crate::tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult, TxTokenTransfer};

use super::{
    super::builtin_func_trait::BuiltinFunction,
    transfer_common::{
        execute_transfer_builtin_func, process_raw_esdt_transfers, ParsedTransferBuiltinFunCall,
        RawEsdtTransfer,
    },
};

pub struct ESDTTransfer;

impl BuiltinFunction for ESDTTransfer {
    fn name(&self) -> &str {
        ESDT_TRANSFER_FUNC_NAME
    }

    fn extract_esdt_transfers(&self, tx_input: &TxInput) -> Vec<TxTokenTransfer> {
        if let Ok(parsed_tx) = try_parse_input(&tx_input) {
            process_raw_esdt_transfers(parsed_tx.raw_esdt_transfers)
        } else {
            Vec::new()
        }
    }

    fn execute(&self, tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate) {
        match try_parse_input(&tx_input) {
            Ok(parsed_tx) => {
                execute_transfer_builtin_func(parsed_tx, self.name(), tx_input, tx_cache)
            },
            Err(message) => {
                let err_result = TxResult::from_vm_error(message.to_string());
                (err_result, BlockchainUpdate::empty())
            },
        }
    }
}

fn try_parse_input(tx_input: &TxInput) -> Result<ParsedTransferBuiltinFunCall, &'static str> {
    if tx_input.args.len() < 2 {
        return Err("ESDTTransfer too few arguments");
    }

    let token_identifier = tx_input.args[0].clone();
    let value_bytes = tx_input.args[1].clone();

    let func_name = tx_input.func_name_from_arg_index(2);
    let args = if tx_input.args.len() > 2 {
        tx_input.args[3..].to_vec()
    } else {
        Vec::new()
    };

    Ok(ParsedTransferBuiltinFunCall {
        destination: tx_input.to.clone(),
        raw_esdt_transfers: vec![RawEsdtTransfer {
            token_identifier,
            nonce_bytes: Vec::new(),
            value_bytes,
        }],
        func_name,
        args,
    })
}
