#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = NavigatorUABrandVersion)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NavigatorUaBrandVersion` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaBrandVersion`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type NavigatorUaBrandVersion;
    #[wasm_bindgen(method, setter = "brand")]
    fn brand_shim(this: &NavigatorUaBrandVersion, val: &str);
    #[wasm_bindgen(method, setter = "version")]
    fn version_shim(this: &NavigatorUaBrandVersion, val: &str);
}
#[cfg(web_sys_unstable_apis)]
impl NavigatorUaBrandVersion {
    #[doc = "Construct a new `NavigatorUaBrandVersion`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaBrandVersion`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `brand` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaBrandVersion`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn brand(&mut self, val: &str) -> &mut Self {
        self.brand_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaBrandVersion`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn version(&mut self, val: &str) -> &mut Self {
        self.version_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for NavigatorUaBrandVersion {
    fn default() -> Self {
        Self::new()
    }
}
