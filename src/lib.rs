extern  crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize, Serializer};
use wasm_bindgen::JsValue;
use js_sys::{Function, Object};
use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &JsValue);

    #[wasm_bindgen(js_namespace = Ext)]
    // fn create(s: &str,conf:&JsValue);
    fn create(s: &str,conf:&JsValue);
    
    #[wasm_bindgen(js_namespace = Ext)]
    // fn create(s: &str,conf:&JsValue);
    fn ext_define(s: &str,conf:&JsValue);
}
#[derive(Serialize)]
struct Grid {
    a: i32,
    id:String
}


struct Application{

}

pub fn define(){

}
#[wasm_bindgen]
pub fn test() {
    let config = Grid{a:1,id:String::from("test")};
    let temp = to_value(&config).unwrap();
    create("Ext.grid.Panel",&temp);
    // ext_define("", );
}
