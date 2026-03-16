use crate::CrossExtismError;

pub struct Plugin { plugin: extism::Plugin }

impl Plugin {
	pub async fn load_url(url: &str) -> Result<Self, CrossExtismError> {
		let plugin = extism::Plugin::new(crossfetch::fetch(url).await?, None, false)?;
		return Ok(Self { plugin });
	} // end fn new
	pub async fn load_bytes(bytes: &[u8]) -> Result<Self, CrossExtismError> {
		let plugin = extism::Plugin::new(bytes, None, false)?;
		return Ok(Self { plugin });
	} // end fn new
	pub async fn call(&mut self, function_name: &str, input: &[u8]) -> Result<Vec<u8>, CrossExtismError> {
		let result = self.plugin.call(function_name, input)?;
		return Ok(result);
	} // end fn call
} // end impl Plugin
