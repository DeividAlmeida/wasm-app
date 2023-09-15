extern crate wasm_bindgen;
mod structs;

use structs::{MyObject, MyObject2, Tag, Total};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
impl MyObject {
    #[wasm_bindgen(constructor)]
    pub fn new(val: Vec<JsValue>) -> MyObject {

      let mut array: MyObject = MyObject(vec![]);

      for item in val.iter() {
        let x: Total  = serde_wasm_bindgen::from_value(item.clone()).unwrap();
          array.0.push(x);
      }
      array
    }

    pub fn get(self) -> JsValue {
      serde_wasm_bindgen::to_value(&self).unwrap()
    }
}