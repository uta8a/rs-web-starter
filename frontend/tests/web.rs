//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use yew::Component;
use yew::prelude::*;
use log::*;
use yew::utils::document;
use frontend::app::*;
use frontend;
use yew::virtual_dom::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn html_test() {
    // let app_test = App::create((), link);
    // mock app
    // https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/browsers.html
    // headless browser test
    // let scope = yew::app::App::new();
    // let main = frontend::app::Model::create((), scope.mount_as_body());
    // main.view();
    // frontend::run_app();
    let doc = yew::App::<Model>::new().mount_to_body().get_component().unwrap().view();
    let doc2 = document();
    // let view_all = doc;
    // let doc2 = document();
    // let v = VNode::VRef(doc2.create_element("div").unwrap().into());
    // assert_eq!(doc, doc2);
    // let doc = document();
    
    // let res = doc.view().query_selector("#root");
    
    // assert_eq!(doc.view(), None);
    assert_eq!(doc2.query_selector("#root").unwrap().unwrap().inner_html(), "helllo".to_string());
}
