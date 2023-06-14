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

//! Autogenerated weights for pallet_omnipool_liquidity_mining
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-05, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --pallet=pallet-omnipool-liquidity-mining
// --extra
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// omnipool_lm.rs
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

use pallet_omnipool_liquidity_mining::weights::WeightInfo;

/// Weights for pallet_omnipool_liquidity_mining using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM FarmSequencer (r:1 w:1)
	// Proof: OmnipoolWarehouseLM FarmSequencer (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Duster AccountBlacklist (r:0 w:1)
	// Proof Skipped: Duster AccountBlacklist (max_values: None, max_size: None, mode: Measured)
	// Storage: OmnipoolWarehouseLM GlobalFarm (r:0 w:1)
	// Proof: OmnipoolWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	fn create_global_farm() -> Weight {
		// Minimum execution time: 35_975 nanoseconds.
		Weight::from_ref_time(36_667_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: OmnipoolWarehouseLM GlobalFarm (r:1 w:1)
	// Proof: OmnipoolWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Duster AccountBlacklist (r:1 w:1)
	// Proof Skipped: Duster AccountBlacklist (max_values: None, max_size: None, mode: Measured)
	fn terminate_global_farm() -> Weight {
		// Minimum execution time: 38_256 nanoseconds.
		Weight::from_ref_time(38_737_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Omnipool Assets (r:1 w:0)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM GlobalFarm (r:1 w:1)
	// Proof: OmnipoolWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM ActiveYieldFarm (r:1 w:1)
	// Proof: OmnipoolWarehouseLM ActiveYieldFarm (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:2 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM FarmSequencer (r:1 w:1)
	// Proof: OmnipoolWarehouseLM FarmSequencer (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM YieldFarm (r:0 w:1)
	// Proof: OmnipoolWarehouseLM YieldFarm (max_values: None, max_size: Some(198), added: 2673, mode: MaxEncodedLen)
	fn create_yield_farm() -> Weight {
		// Minimum execution time: 65_699 nanoseconds.
		Weight::from_ref_time(67_132_000 as u64)
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Omnipool Assets (r:1 w:0)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM ActiveYieldFarm (r:1 w:0)
	// Proof: OmnipoolWarehouseLM ActiveYieldFarm (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM YieldFarm (r:1 w:1)
	// Proof: OmnipoolWarehouseLM YieldFarm (max_values: None, max_size: Some(198), added: 2673, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM GlobalFarm (r:1 w:1)
	// Proof: OmnipoolWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:2 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	fn update_yield_farm() -> Weight {
		// Minimum execution time: 68_111 nanoseconds.
		Weight::from_ref_time(69_194_000 as u64)
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: OmnipoolWarehouseLM ActiveYieldFarm (r:1 w:1)
	// Proof: OmnipoolWarehouseLM ActiveYieldFarm (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM YieldFarm (r:1 w:1)
	// Proof: OmnipoolWarehouseLM YieldFarm (max_values: None, max_size: Some(198), added: 2673, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM GlobalFarm (r:1 w:1)
	// Proof: OmnipoolWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:2 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	fn stop_yield_farm() -> Weight {
		// Minimum execution time: 62_854 nanoseconds.
		Weight::from_ref_time(63_400_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Omnipool Assets (r:1 w:0)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM ActiveYieldFarm (r:1 w:1)
	// Proof: OmnipoolWarehouseLM ActiveYieldFarm (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM YieldFarm (r:1 w:1)
	// Proof: OmnipoolWarehouseLM YieldFarm (max_values: None, max_size: Some(198), added: 2673, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM GlobalFarm (r:1 w:1)
	// Proof: OmnipoolWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:2 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	fn resume_yield_farm() -> Weight {
		// Minimum execution time: 66_977 nanoseconds.
		Weight::from_ref_time(67_559_000 as u64)
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: OmnipoolWarehouseLM ActiveYieldFarm (r:1 w:0)
	// Proof: OmnipoolWarehouseLM ActiveYieldFarm (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM GlobalFarm (r:1 w:1)
	// Proof: OmnipoolWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM YieldFarm (r:1 w:1)
	// Proof: OmnipoolWarehouseLM YieldFarm (max_values: None, max_size: Some(198), added: 2673, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn terminate_yield_farm() -> Weight {
		// Minimum execution time: 35_883 nanoseconds.
		Weight::from_ref_time(36_197_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Uniques Asset (r:2 w:2)
	// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	// Storage: Omnipool Positions (r:1 w:0)
	// Proof: Omnipool Positions (max_values: None, max_size: Some(100), added: 2575, mode: MaxEncodedLen)
	// Storage: Omnipool Assets (r:1 w:0)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM YieldFarm (r:1 w:1)
	// Proof: OmnipoolWarehouseLM YieldFarm (max_values: None, max_size: Some(198), added: 2673, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM GlobalFarm (r:1 w:1)
	// Proof: OmnipoolWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:4 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM DepositSequencer (r:1 w:1)
	// Proof: OmnipoolWarehouseLM DepositSequencer (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Uniques Class (r:2 w:1)
	// Proof: Uniques Class (max_values: None, max_size: Some(190), added: 2665, mode: MaxEncodedLen)
	// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	// Proof: Uniques CollectionMaxSupply (max_values: None, max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	// Storage: Uniques Account (r:0 w:3)
	// Proof: Uniques Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(113), added: 2588, mode: MaxEncodedLen)
	// Storage: OmnipoolLiquidityMining OmniPositionId (r:0 w:1)
	// Proof: OmnipoolLiquidityMining OmniPositionId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM Deposit (r:0 w:1)
	// Proof: OmnipoolWarehouseLM Deposit (max_values: None, max_size: Some(385), added: 2860, mode: MaxEncodedLen)
	fn deposit_shares() -> Weight {
		// Minimum execution time: 115_282 nanoseconds.
		Weight::from_ref_time(116_568_000 as u64)
			.saturating_add(T::DbWeight::get().reads(17 as u64))
			.saturating_add(T::DbWeight::get().writes(14 as u64))
	}
	// Storage: Uniques Asset (r:2 w:0)
	// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	// Storage: OmnipoolLiquidityMining OmniPositionId (r:1 w:0)
	// Proof: OmnipoolLiquidityMining OmniPositionId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	// Storage: Omnipool Positions (r:1 w:0)
	// Proof: Omnipool Positions (max_values: None, max_size: Some(100), added: 2575, mode: MaxEncodedLen)
	// Storage: Omnipool Assets (r:1 w:0)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM Deposit (r:1 w:1)
	// Proof: OmnipoolWarehouseLM Deposit (max_values: None, max_size: Some(385), added: 2860, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM YieldFarm (r:1 w:1)
	// Proof: OmnipoolWarehouseLM YieldFarm (max_values: None, max_size: Some(198), added: 2673, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM GlobalFarm (r:1 w:1)
	// Proof: OmnipoolWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:4 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	fn redeposit_shares() -> Weight {
		// Minimum execution time: 101_503 nanoseconds.
		Weight::from_ref_time(102_925_000 as u64)
			.saturating_add(T::DbWeight::get().reads(15 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Uniques Asset (r:1 w:0)
	// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM Deposit (r:1 w:1)
	// Proof: OmnipoolWarehouseLM Deposit (max_values: None, max_size: Some(385), added: 2860, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM YieldFarm (r:1 w:1)
	// Proof: OmnipoolWarehouseLM YieldFarm (max_values: None, max_size: Some(198), added: 2673, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM GlobalFarm (r:1 w:1)
	// Proof: OmnipoolWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:3 w:3)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:2 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	fn claim_rewards() -> Weight {
		// Minimum execution time: 80_025 nanoseconds.
		Weight::from_ref_time(81_388_000 as u64)
			.saturating_add(T::DbWeight::get().reads(10 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Uniques Asset (r:2 w:2)
	// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	// Storage: OmnipoolLiquidityMining OmniPositionId (r:1 w:1)
	// Proof: OmnipoolLiquidityMining OmniPositionId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	// Storage: Omnipool Positions (r:1 w:0)
	// Proof: Omnipool Positions (max_values: None, max_size: Some(100), added: 2575, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM Deposit (r:1 w:1)
	// Proof: OmnipoolWarehouseLM Deposit (max_values: None, max_size: Some(385), added: 2860, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM YieldFarm (r:1 w:1)
	// Proof: OmnipoolWarehouseLM YieldFarm (max_values: None, max_size: Some(198), added: 2673, mode: MaxEncodedLen)
	// Storage: OmnipoolWarehouseLM GlobalFarm (r:1 w:1)
	// Proof: OmnipoolWarehouseLM GlobalFarm (max_values: None, max_size: Some(205), added: 2680, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:3 w:3)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:2 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	// Storage: Uniques Class (r:2 w:1)
	// Proof: Uniques Class (max_values: None, max_size: Some(190), added: 2665, mode: MaxEncodedLen)
	// Storage: Uniques Account (r:0 w:3)
	// Proof: Uniques Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	// Storage: Uniques ItemPriceOf (r:0 w:2)
	// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(113), added: 2588, mode: MaxEncodedLen)
	fn withdraw_shares() -> Weight {
		// Minimum execution time: 128_023 nanoseconds.
		Weight::from_ref_time(129_370_000 as u64)
			.saturating_add(T::DbWeight::get().reads(15 as u64))
			.saturating_add(T::DbWeight::get().writes(15 as u64))
	}
}
