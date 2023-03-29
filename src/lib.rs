use std::collections::HashMap;

use wasm_bindgen::prelude::*;

#[derive(serde::Deserialize, Debug)]
struct PFInfo {
    name: String
}

#[wasm_bindgen]
pub fn get_pf_map_from_js(object: JsValue) -> String {
    let object = serde_wasm_bindgen::from_value::<HashMap<String, PFInfo>>(object);

    match object {
        Ok(e) => format!("ok {:?}", e),
        Err(e) => format!("error {:?}", e),
    }
}
