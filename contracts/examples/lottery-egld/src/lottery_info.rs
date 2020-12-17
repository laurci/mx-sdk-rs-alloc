use elrond_wasm::elrond_codec::*;
use elrond_wasm::{Address, BigUintApi, Vec};

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct LotteryInfo<BigUint: BigUintApi> {
	pub ticket_price: BigUint,
	pub tickets_left: u32,
	pub deadline: u64,
	pub max_entries_per_user: u32,
	pub prize_distribution: Vec<u8>,
	pub whitelist: Vec<Address>,
	pub current_ticket_number: u32,
	pub prize_pool: BigUint,
}