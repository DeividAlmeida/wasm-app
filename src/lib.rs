extern crate wasm_bindgen;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use js_sys::{Reflect, JSON, JsString};
use wasm_bindgen::JsValue;

#[wasm_bindgen]
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct MyObject {
  pub id: u8,
  name: String,
  username: String,
  email: String,
  array: Option<Vec<MyObject>>
}

#[wasm_bindgen]
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct MyArray {
  array: Vec<MyObject>
}

#[wasm_bindgen]
impl MyObject {
    #[wasm_bindgen(constructor)]
    pub fn new(val: Vec<JsValue>) -> MyObject {

      let mut array: Vec<MyObject> = Vec::new();

      for item in val.iter() {
        let x: MyObject  = serde_wasm_bindgen::from_value(item.clone()).unwrap();

        let obj = MyObject {
          id: x.id,
          name: x.name,
          username: x.username,
          email: x.email,
          array: None
        };
        array.push(obj);
      }

      MyObject {
        id: 0,
        name: "test".to_string(),
        username: "test".to_string(),
        email: "test".to_string(),
        array: Some(array)
      }
      
    }

    pub fn get(self) -> JsValue {
      serde_wasm_bindgen::to_value(&self).unwrap()
    }

    // pub fn set(&mut self, val: MyObject) -> JsValue  {
    //   self.array.id = val.id;
    //   self.name = val.name;
    //   self.username = val.username;
    //   self.email = val.email;
    //   self.get()
    // }
}