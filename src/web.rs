use crate::CrossExtismError;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::Promise;
use js_sys::Reflect;

#[wasm_bindgen(module = "https://esm.sh/@extism/extism@2.0.0-rc13/es2022/extism.bundle.mjs")]
unsafe extern "C" {
	#[wasm_bindgen(js_name = "createPlugin")]
	pub unsafe fn create_plugin(manifest: JsValue, config: JsValue) -> Promise;
} // end extern "C"

pub struct Plugin { plugin: JsValue, call_fn: js_sys::Function }

impl Plugin {
	pub async fn load_url(url: &str) -> Result<Self, CrossExtismError> {
		let manifest = js_sys::JsString::from(url);
		let config = JsValue::UNDEFINED;
		let promise = unsafe { create_plugin(manifest.into(), config) };
		let value = JsFuture::from(promise).await?;
		let call_fn = Reflect::get(&value, &JsValue::from_str("call"))?.dyn_into::<js_sys::Function>()?;
		return Ok(Self { plugin: value, call_fn });
	} // end fn new
	pub async fn load_bytes(bytes: &[u8]) -> Result<Self, CrossExtismError> {
		let manifest: JsValue = js_sys::Uint8Array::from(bytes).into();
		let config = JsValue::UNDEFINED;
		let promise = unsafe { create_plugin(manifest.into(), config) };
		let value = JsFuture::from(promise).await?;
		let call_fn = Reflect::get(&value, &JsValue::from_str("call"))?.dyn_into::<js_sys::Function>()?;
		return Ok(Self { plugin: value, call_fn });
	} // end fn new
	pub async fn call(&mut self, function_name: &str, input: &[u8]) -> Result<Vec<u8>, CrossExtismError> {
		let uint8array = unsafe { js_sys::Uint8Array::view(input) };
		let result_promise: Promise = self.call_fn.call2(&self.plugin, &JsValue::from_str(function_name), &uint8array)?.dyn_into()?;
		let result_js_value = JsFuture::from(result_promise).await?;
		let data_view: js_sys::DataView = result_js_value.dyn_into()?;
		let byte_offset = u32::try_from(data_view.byte_offset()).map_err(|_| JsValue::from_str("DataView byte_offset exceeds u32"))?;
		let byte_length = u32::try_from(data_view.byte_length()).map_err(|_| JsValue::from_str("DataView byte_length exceeds u32"))?;
		let result_uint8array = js_sys::Uint8Array::new_with_byte_offset_and_length(&data_view.buffer(), byte_offset, byte_length);
		return Ok(result_uint8array.to_vec());
	} // end fn call
} // end impl Plugin

