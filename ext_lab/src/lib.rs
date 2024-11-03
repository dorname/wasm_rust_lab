extern  crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize, Serializer};
use wasm_bindgen::JsValue;
use js_sys::{Function, Object,Reflect};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;
use web_sys::{Window, Event, HtmlElement};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &JsValue);

    #[wasm_bindgen(js_namespace = Ext)]
    // fn create(s: &str,conf:&JsValue);
    fn create(s: &str,conf:&JsValue);

    #[wasm_bindgen(js_namespace = Ext)]
    // fn create(s: &str,conf:&JsValue);
    fn getBody()-> JsValue;
    
    #[wasm_bindgen(js_namespace = Ext)]
    // fn create(s: &str,conf:&JsValue);
    fn ext_define(s: &str,conf:&JsValue);
}


#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");

    // 获取某个元素，例如一个按钮
    let button = document.get_element_by_id("myButton")
        .expect("document should have a button with ID `myButton`")
        .dyn_into::<HtmlElement>()
        .map_err(|_| JsValue::from_str("Could not convert to HtmlElement"))?;

    // 创建一个闭包处理点击事件
    let closure = Closure::wrap(Box::new(move |event: Event| {
        alert("Button was clicked!");
    }) as Box<dyn FnMut(Event)>);

    // 添加事件监听器
    button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;

    // 必须忘记闭包以避免 Rust 清除它，因为它还将在 JS 中使用
    closure.forget();

    Ok(())
}

fn alert(message: &str) {
    window().unwrap().alert_with_message(message).unwrap();
}

pub fn define(){

}


#[wasm_bindgen]
pub fn log_example() {
    let message = JsValue::from("Hello from Rust and WebAssembly!");
    log(&message);
}

#[wasm_bindgen]
pub fn test() {
        // 创建一个 JavaScript 对象
        let config = Object::new();

        let body = getBody();

        // 设置对象的属性，比如 `id` 和 `renderTo`
        Reflect::set(&config, &JsValue::from_str("id"), &JsValue::from_str("myGrid"))
            .expect("Failed to set property `id`");
    
        // 假设 `renderTo` 是一个 DOM 元素 ID
        Reflect::set(&config, &JsValue::from_str("renderTo"), &body)
            .expect("Failed to set property `renderTo`");
    
        // // 设置其他属性（根据 ExtJS 的需求）
        Reflect::set(&config, &JsValue::from_str("width"), &JsValue::from_f64(400.0))
            .expect("Failed to set property `width`");
        Reflect::set(&config, &JsValue::from_str("height"), &JsValue::from_f64(300.0))
            .expect("Failed to set property `height`");
        Reflect::set(&config, &JsValue::from_str("title"), &JsValue::from_str("ceshi"))
        .expect("Failed to set property `height`");
    
        // 调用 `Ext.create`，传递类名和配置对象
        create("Ext.grid.Panel", &config.into());
}