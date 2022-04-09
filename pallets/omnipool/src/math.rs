pub mod hydradx_math {
	use crate::{AssetState, Config, FixedU128};
	use sp_runtime::traits::{CheckedAdd, CheckedMul, CheckedSub};
	use sp_runtime::FixedPointNumber;

	pub struct StateChanges<Balance> {
		pub delta_reserve_in: Balance,
		pub delta_reserve_out: Balance,
		pub delta_hub_reserve_in: Balance,
		pub delta_hub_reserve_out: Balance,
	}

	pub fn calculate_sell_state_changes<T: Config>(
		asset_in_state: &AssetState<T::Balance>,
		asset_out_state: &AssetState<T::Balance>,
		amount: T::Balance,
		asset_fee: FixedU128,
		protocol_fee: FixedU128,
	) -> Option<StateChanges<T::Balance>> {
		let delta_hub_reserve_in = FixedU128::from((amount, (asset_in_state.reserve.checked_add(&amount)?)))
			.checked_mul_int(asset_in_state.hub_reserve)?;

		let fee_p = FixedU128::from(1).checked_sub(&protocol_fee)?;

		let delta_hub_reserve_out = fee_p.checked_mul_int(delta_hub_reserve_in)?;

		let fee_a = FixedU128::from(1).checked_sub(&asset_fee)?;

		let hub_reserve_out = asset_out_state.hub_reserve.checked_add(&delta_hub_reserve_out)?;

		let delta_reserve_out = FixedU128::from((delta_hub_reserve_out, hub_reserve_out))
			.checked_mul(&fee_a)
			.and_then(|v| v.checked_mul_int(asset_out_state.reserve))?;

		Some(StateChanges {
			delta_reserve_in: amount,
			delta_reserve_out,
			delta_hub_reserve_in,
			delta_hub_reserve_out,
		})
	}

	#[allow(unused)]
	fn calculate_out_given_in<Balance: Default>() -> Balance {
		Balance::default()
	}

	#[allow(unused)]
	fn calculate_in_given_out<Balance: Default>() -> Balance {
		Balance::default()
	}

	#[allow(unused)]
	fn calculate_shares_given_liquidity_in<Balance: Default>() -> Balance {
		Balance::default()
	}
}
