#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
	pub trait IbcApi {
		fn consensus_state_with_height(client_id: Vec<u8>) -> Vec<(Vec<u8>, Vec<u8>)>;
	}
}