#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub use web::*;

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[derive(Debug)]
pub enum CrossExtismError {
	#[cfg(not(target_arch = "wasm32"))]
	ExtismError(extism::Error),
	#[cfg(not(target_arch = "wasm32"))]
	CrossFetchError(crossfetch::CrossFetchError),
	#[cfg(target_arch = "wasm32")]
	JSError(wasm_bindgen::JsValue),
} // end enum Error

#[cfg(not(target_arch = "wasm32"))]
impl From<extism::Error> for CrossExtismError {
	fn from(error: extism::Error) -> Self {
		return Self::ExtismError(error);
	} // end fn from
} // end impl From<extism::Error> for CrossExtismError

#[cfg(not(target_arch = "wasm32"))]
impl From<crossfetch::CrossFetchError> for CrossExtismError {
	fn from(error: crossfetch::CrossFetchError) -> Self {
		return Self::CrossFetchError(error);
	} // end fn from
} // end impl From<crossfetch::CrossFetchError> for CrossExtismError

#[cfg(target_arch = "wasm32")]
impl From<wasm_bindgen::JsValue> for CrossExtismError {
	fn from(error: wasm_bindgen::JsValue) -> Self {
		return Self::JSError(error);
	} // end fn from
} // end impl From<wasm_bindgen::JsValue> for CrossExtismError

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
	use super::*;
	#[tokio::test]
	async fn test_plugin() {
		let mut plugin = Plugin::load_url("https://cdn.modsurfer.dylibso.com/api/v1/module/be716369b7332148771e3cd6376d688dfe7ee7dd503cbc43d2550d76cb45a01d.wasm").await.unwrap();
		let result = plugin.call("count_vowels", b"Hello, World!").await.unwrap();
		let result_str = String::from_utf8(result).unwrap();
		assert_eq!(result_str, r#"{"count":3,"total":3,"vowels":"aeiouAEIOU"}"#);
	} // end fn test_plugin
} // end mod tests

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod tests {
	// Note: These tests are meant to be run in a browser environment using `wasm-pack test --headless --firefox`
	use super::*;
	use wasm_bindgen_test::*;
	#[wasm_bindgen_test]
	async fn test_plugin() {
		let mut plugin = Plugin::load_url("https://cdn.modsurfer.dylibso.com/api/v1/module/be716369b7332148771e3cd6376d688dfe7ee7dd503cbc43d2550d76cb45a01d.wasm").await.unwrap();
		let result = plugin.call("count_vowels", b"Hello, World!").await.unwrap();
		let result_str = String::from_utf8(result).unwrap();
		assert_eq!(result_str, r#"{"count":3,"total":3,"vowels":"aeiouAEIOU"}"#);
	} // end fn test_plugin
	wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
} // end mod tests