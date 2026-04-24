#[cfg(not(target_arch = "wasm32"))]
use leptos::prelude::RenderHtml;
#[cfg(target_arch = "wasm32")]
use leptos::prelude::*;
use sql_intelliscan_ui::App;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, JsValue};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test;
#[cfg(target_arch = "wasm32")]
use web_sys::{Event, HtmlButtonElement, HtmlInputElement};

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn given_public_application_contract_when_rendering_and_reading_runtime_copy_then_integration_should_validate_complete_basic_experience()
{
    let rendered_html = App().to_html();

    assert!(rendered_html.contains("<h1>Welcome to SQL Intelliscan</h1>"));
}

#[cfg(target_arch = "wasm32")]
async fn flush_ui_updates() {
    let _ = JsFuture::from(js_sys::Promise::resolve(&JsValue::NULL)).await;
    let _ = JsFuture::from(js_sys::Promise::resolve(&JsValue::NULL)).await;
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test(async)]
async fn given_greet_form_when_entering_prueba_and_clicking_submit_then_should_render_rust_greeting()
{
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("window should be available");
    let document = window.document().expect("document should be available");

    mount_to_body(|| view! { <App /> });

    let input = document
        .get_element_by_id("greet-input")
        .expect("greet input should exist")
        .dyn_into::<HtmlInputElement>()
        .expect("greet input should be an input element");
    input.set_value("prueba");
    input
        .dispatch_event(&Event::new("input").expect("input event should be created"))
        .expect("input event should dispatch");

    let button = document
        .get_element_by_id("greet-button")
        .expect("greet button should exist")
        .dyn_into::<HtmlButtonElement>()
        .expect("greet button should be a button element");
    button.click();

    flush_ui_updates().await;

    let body_text = document
        .body()
        .and_then(|body| body.text_content())
        .expect("document body should expose text content");

    assert!(
        body_text.contains("Hello, prueba! You've been greeted from Rust!"),
        "The greeting message should be rendered after submitting the form"
    );
}
