extern crate wasm_bindgen;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct MyObject (
  pub (crate) Vec<Total>
);

#[wasm_bindgen]
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct Tag {
  code: String,
  version: String,
  manufactorer: String,
  deviceModel: String,
}

#[wasm_bindgen]
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct Manusis {
  asset_id: String,
  temperature_pp_tag: String,
  battery_pp_tag: String,
  position_pp_tag: String,
}

#[wasm_bindgen]
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct ActionButtonSimplePack {
  short_press_label: String,
  double_click_label: String,
  long_press_label: String,
}

#[wasm_bindgen]
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct Family {
  _id: String,
  code: String,
  company: String,
  created_at: String, // Pode ser uma data, dependendo da biblioteca que você estiver usando
  update_at: String,  // Pode ser uma data, dependendo da biblioteca que você estiver usando
  __v: i32,
}

#[wasm_bindgen]
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct LastPosition {
  _id: String,
  tag: String,
  date: String, // Pode ser uma data, dependendo da biblioteca que você estiver usando
  timestamp: i64,
  latitude: f64,
  longitude: f64,
  accuracy: i32,
  created_at: String, // Pode ser uma data, dependendo da biblioteca que você estiver usando
  __v: i32,
}

#[wasm_bindgen]
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct LastBattery {
  _id: String,
  tag: String,
  date: String, // Pode ser uma data, dependendo da biblioteca que você estiver usando
  timestamp: i64,
  battery: i32,
  batteryVoltage: f64,
  created_at: String, // Pode ser uma data, dependendo da biblioteca que você estiver usando
  __v: i32,
}

#[wasm_bindgen]
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct LastCurrentStateHistory {
  _id: String,
  packing: String,
  r#type: String,
  device_data_id: Option<String>, // Pode ser uma data, dependendo da biblioteca que você estiver usando
  created_at: String,    // Pode ser uma data, dependendo da biblioteca que você estiver usando
  update_at: String,     // Pode ser uma data, dependendo da biblioteca que você estiver usando
  __v: i32,
}

#[wasm_bindgen]
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct LastEventRecord {
  _id: String,
  packing: String,
  control_point: String,
  distance_km: f64,
  accuracy: i32,
  r#type: String,
  created_at: String,    // Pode ser uma data, dependendo da biblioteca que você estiver usando
  device_data_id: String, // Pode ser uma data, dependendo da biblioteca que você estiver usando
  update_at: String,     // Pode ser uma data, dependendo da biblioteca que você estiver usando
  __v: i32,
}

#[wasm_bindgen]
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct MyObject2 {
  tag: Tag,
  manusis: Manusis,
  actionButtonSimplePack: ActionButtonSimplePack,
  _id: String,
  weigth: i32,
  width: i32,
  heigth: i32,
  length: i32,
  capacity: i32,
  temperature: i32,
  active: bool,
  absent: bool,
  absent_time: Option<String>, // Pode ser uma data, dependendo da biblioteca que você estiver usando
  cicle_start: String,         // Pode ser uma data, dependendo da biblioteca que você estiver usando
  cicle_end: String,           // Pode ser uma data, dependendo da biblioteca que você estiver usando
  last_cicle_duration: f64,
  low_battery: bool,
  permanence_time_exceeded: bool,
  last_message_signal: String, // Pode ser uma data, dependendo da biblioteca que você estiver usando
  current_state: String,
  max_production_parts: i32,
  actual_production: i32,
  fixed_asset: bool,
  serial: String,
  r#type: String,
  family: Family,
  project: String,
  offlineWhileAbsent: Vec<OfflineWhileAbsent>,
  created_at: String, // Pode ser uma data, dependendo da biblioteca que você estiver usando
  update_at: String,  // Pode ser uma data, dependendo da biblioteca que você estiver usando
  __v: i32,
  last_position: LastPosition,
  last_battery: LastBattery,
  last_current_state_history: LastCurrentStateHistory,
  last_event_record: LastEventRecord,
  last_owner_supplier: String,
  last_travel: String,
  observations: String,
  first_attempt_incorrect_local: Option<String>, // Pode ser uma data, dependendo da biblioteca que você estiver usando
  last_temperature: String,
  detector_switch: Option<bool>,
}

#[wasm_bindgen]
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct OfflineWhileAbsent {
  start: String,
  end: Option<String>,
  _id: String,
}

#[wasm_bindgen]
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct Total {
  tag: Tag,
  // // manusis: Manusis,
  // // actionButtonSimplePack: ActionButtonSimplePack,
  // // family: Family,
  // // last_battery: LastBattery,
  // // last_position: LastPosition,
  // // last_event_record: LastEventRecord,
  // // last_current_state_history: LastCurrentStateHistory,
}