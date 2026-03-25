use extism::{Manifest, PluginBuilder, Wasm};
use crate::CrossExtismError;

pub struct Plugin { plugin: extism::Plugin }

impl Plugin {
	pub async fn load_url(url: &str) -> Result<Self, CrossExtismError> {
		let wurl = Wasm::url(url);
		return load_generic(wurl).await;
	} // end fn new
	pub async fn load_bytes(bytes: &[u8]) -> Result<Self, CrossExtismError> {
		let wbytes = Wasm::data(bytes);
		return load_generic(wbytes).await;
	} // end fn new
	pub async fn call<'a, T>(&mut self, function_name: &str, input: T) -> Result<Vec<u8>, CrossExtismError> where T: extism::ToBytes<'a> {
		let result = self.plugin.call(function_name, input)?;
		return Ok(result);
	} // end fn call
} // end impl Plugin

async fn load_generic(wasm: Wasm) -> Result<Plugin, CrossExtismError> {
	let manifest = Manifest::new([wasm]);
	let plugin_builder = PluginBuilder::new(manifest).with_wasi(false);
	let plugin = plugin_builder.build()?;
	return Ok(Plugin { plugin });
} // end fn load_generic

