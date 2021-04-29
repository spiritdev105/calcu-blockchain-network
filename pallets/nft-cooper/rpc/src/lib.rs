use std::sync::Arc;
use sp_blockchain::HeaderBackend;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_runtime::{generic::BlockId, traits::{Block as BlockT}};
use sp_api::ProvideRuntimeApi;
pub use nft_cooper_rpc_runtime_api::NFTCooperApi as NFTCooperRuntimeApi;

#[rpc]
pub trait NFTCooperApi {
	#[rpc(name = "nft_cooper_mintTokenDeposit")]
	fn mint_token_deposit(&self, metadata_len: u32, quantity: u32) -> Result<(String, String)>;

	#[rpc(name = "nft_cooper_createClassDeposit")]
	fn create_class_deposit(&self, metadata_len: u32, name_len: u32, description_len: u32) -> Result<(String, String)>;

	#[rpc(name = "nft_cooper_addClassAdminDeposit")]
	fn add_class_admin_deposit(&self, admin_count: u32) -> Result<String>;
}

/// A struct that implements the [`NFTCooperApi`].
pub struct NFTCooper<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> NFTCooper<C, P> {
	/// Create new `NFTCooper` with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i64 {
	fn from(e: Error) -> i64 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

impl<C, Block> NFTCooperApi for NFTCooper<C, Block>
where
	Block: BlockT,
	C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: NFTCooperRuntimeApi<Block>,
{
	/*
		$ curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
		 "jsonrpc":"2.0",
		  "id":1,
		  "method":"nft_cooper_mintTokenDeposit",
		  "params": [4, 3]
		}'
		{"jsonrpc":"2.0","result":["1040000000000","3120000000000"],"id":1}
		$ websocat ws://localhost:9944
		{"id":1,"jsonrpc":"2.0","method":"nft_cooper_mintTokenDeposit","params":[4, 3]}
		{"jsonrpc":"2.0","result":["1040000000000","3120000000000"],"id":1}
	 */
	fn mint_token_deposit(&self, metadata_len: u32, quantity: u32) -> Result<(String, String)> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(self.client.info().best_hash);
		api.mint_token_deposit(&at, metadata_len, quantity).map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query dispatch info.".into(),
			data: Some(format!("{:?}", e).into()),
		}).map(|(deposit, total_deposit)|{
			(format!("{}", deposit), format!("{}", total_deposit))
		})
	}

	fn create_class_deposit(&self, metadata_len: u32, name_len: u32, description_len: u32) -> Result<(String, String)> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(self.client.info().best_hash);
		api.create_class_deposit(&at, metadata_len, name_len, description_len).map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query dispatch info.".into(),
			data: Some(format!("{:?}", e).into()),
		}).map(|(deposit, total_deposit)|{
			(format!("{}", deposit), format!("{}", total_deposit))
		})
	}

	fn add_class_admin_deposit(&self, admin_count: u32) -> Result<String> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(self.client.info().best_hash);
		api.add_class_admin_deposit(&at, admin_count).map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query dispatch info.".into(),
			data: Some(format!("{:?}", e).into()),
		}).map(|deposit|{
			format!("{}", deposit)
		})
	}
}
