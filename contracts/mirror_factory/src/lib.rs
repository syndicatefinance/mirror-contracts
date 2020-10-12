pub mod contract;
pub mod math;
pub mod msg;
pub mod register_msgs;
pub mod state;

#[cfg(test)]
mod testing;

#[cfg(test)]
mod mock_querier;

#[cfg(target_arch = "wasm32")]
cosmwasm_std::create_entry_points!(contract);
