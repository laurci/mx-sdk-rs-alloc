// Code generated by the mx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           19
// Async Callback (empty):               1
// Total number of exported functions:  21

#![no_std]
#![feature(alloc_error_handler, lang_items)]

mx_sc_wasm_adapter::allocator!();
mx_sc_wasm_adapter::panic_handler!();

mx_sc_wasm_adapter::endpoints! {
    mx_price_aggregator_sc
    (
        addOracles
        removeOracles
        submit
        submitBatch
        latestRoundData
        latestPriceFeed
        latestPriceFeedOptional
        setSubmissionCount
        getOracles
        setPairDecimals
        getPairDecimals
        submission_count
        pause
        unpause
        isPaused
        stake
        unstake
        voteSlashMember
        slashMember
    )
}

mx_sc_wasm_adapter::empty_callback! {}
