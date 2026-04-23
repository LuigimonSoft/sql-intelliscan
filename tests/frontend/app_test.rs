
use leptos::prelude::*;
use leptos::web_sys;
use sql_intelliscan_ui::app::App;
use wasm_bindgen_test::wasm_bindgen_test;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn given_app_component_when_mounted_then_should_render_h1_should_render_h1_component() {
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("window should be available");
    let document = window.document().expect("document should be available");
    mount_to_body(|| view! { <App /> });

    let title = document
        .query_selector("h1")
        .expect("selector should not fail");

    assert!(title.is_some(), "App should render an <h1> element");
    assert_eq!(
        title.unwrap().text_content().unwrap(),
        "Welcome to SQL Intelliscan",
        "The <h1> element should contain the correct text"
    );
}
