// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           15
// Async Callback (empty):               1
// Total number of exported functions:  17

#![no_std]
#![feature(alloc_error_handler, lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    vault
    (
        echo_arguments
        echo_arguments_without_storage
        echo_caller
        accept_funds
        accept_funds_echo_payment
        accept_funds_single_esdt_transfer
        reject_funds
        retrieve_funds_with_transfer_exec
        retrieve_funds_promises
        retrieve_funds
        retrieve_multi_funds_async
        burn_and_create_retrive_async
        get_owner_address
        call_counts
        num_called_retrieve_funds_promises
    )
}

multiversx_sc_wasm_adapter::empty_callback! {}
