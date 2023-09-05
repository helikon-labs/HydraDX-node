// This file is part of HydraDX.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_stableswap
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-17, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --pallet=pallet-stableswap
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// stableswap.rs
// --template
// .maintain/pallet-weight-template-no-back.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use pallet_stableswap::weights::WeightInfo;

/// Weights for pallet_stableswap using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	// Storage: Stableswap Pools (r:1 w:1)
	// Proof: Stableswap Pools (max_values: None, max_size: Some(61), added: 2536, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:6 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: Duster AccountBlacklist (r:0 w:1)
	// Proof: Duster AccountBlacklist (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn create_pool() -> Weight {
		// Minimum execution time: 52_216 nanoseconds.
		Weight::from_ref_time(52_934_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Stableswap Pools (r:1 w:0)
	// Proof: Stableswap Pools (max_values: None, max_size: Some(61), added: 2536, mode: MaxEncodedLen)
	// Storage: Stableswap AssetTradability (r:5 w:0)
	// Proof: Stableswap AssetTradability (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:11 w:11)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:6 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:0)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	fn add_liquidity() -> Weight {
		// Minimum execution time: 571_271 nanoseconds.
		Weight::from_ref_time(676_183_000 as u64)
			.saturating_add(T::DbWeight::get().reads(27 as u64))
			.saturating_add(T::DbWeight::get().writes(13 as u64))
	}
	// Storage: Stableswap AssetTradability (r:1 w:0)
	// Proof: Stableswap AssetTradability (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:7 w:3)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Stableswap Pools (r:1 w:0)
	// Proof: Stableswap Pools (max_values: None, max_size: Some(61), added: 2536, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:0 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn remove_liquidity_one_asset() -> Weight {
		// Minimum execution time: 401_601 nanoseconds.
		Weight::from_ref_time(402_817_000 as u64)
			.saturating_add(T::DbWeight::get().reads(15 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Stableswap AssetTradability (r:2 w:0)
	// Proof: Stableswap AssetTradability (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:7 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Stableswap Pools (r:1 w:0)
	// Proof: Stableswap Pools (max_values: None, max_size: Some(61), added: 2536, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:0 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn sell() -> Weight {
		// Minimum execution time: 356_780 nanoseconds.
		Weight::from_ref_time(357_640_000 as u64)
			.saturating_add(T::DbWeight::get().reads(15 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Stableswap AssetTradability (r:2 w:0)
	// Proof: Stableswap AssetTradability (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	// Storage: Stableswap Pools (r:1 w:0)
	// Proof: Stableswap Pools (max_values: None, max_size: Some(61), added: 2536, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:7 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:0)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	fn buy() -> Weight {
		// Minimum execution time: 341_023 nanoseconds.
		Weight::from_ref_time(343_079_000 as u64)
			.saturating_add(T::DbWeight::get().reads(16 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Stableswap Pools (r:1 w:0)
	// Proof: Stableswap Pools (max_values: None, max_size: Some(61), added: 2536, mode: MaxEncodedLen)
	// Storage: Stableswap AssetTradability (r:1 w:1)
	// Proof: Stableswap AssetTradability (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	fn set_asset_tradable_state() -> Weight {
		// Minimum execution time: 23_927 nanoseconds.
		Weight::from_ref_time(24_426_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Stableswap Pools (r:1 w:1)
	// Proof: Stableswap Pools (max_values: None, max_size: Some(61), added: 2536, mode: MaxEncodedLen)
	fn update_pool_fee() -> Weight {
		// Minimum execution time: 22_216 nanoseconds.
		Weight::from_ref_time(22_460_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Stableswap Pools (r:1 w:1)
	// Proof: Stableswap Pools (max_values: None, max_size: Some(61), added: 2536, mode: MaxEncodedLen)
	fn update_amplification() -> Weight {
		// Minimum execution time: 23_815 nanoseconds.
		Weight::from_ref_time(24_353_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
