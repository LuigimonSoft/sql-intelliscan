#![allow(non_snake_case)]

#[cfg(not(target_arch = "wasm32"))]
use leptos::prelude::*;
#[cfg(target_arch = "wasm32")]
use leptos::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use leptos::prelude::RenderHtml;
#[cfg(target_arch = "wasm32")]
use leptos::web_sys;
#[cfg(not(target_arch = "wasm32"))]
use sql_intelliscan_ui::app::{
    greet_message, greet_message_sync, invoke_greet_sync, should_send_greet, spawn_greet, App,
};
#[cfg(target_arch = "wasm32")]
use sql_intelliscan_ui::app::App;
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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn GivenAppComponent_WhenMounted_ThenH1_ShouldRenderExpectedTitle() {
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

#[cfg(target_arch = "wasm32")]
async fn flush_ui_updates() {
    let _ = JsFuture::from(js_sys::Promise::resolve(&JsValue::NULL)).await;
    let _ = JsFuture::from(js_sys::Promise::resolve(&JsValue::NULL)).await;
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test(async)]
async fn GivenGreetForm_WhenSubmittingName_ThenMessage_ShouldRenderRustGreeting() {
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

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn GivenName_WhenInvokeGreetSyncIsCalled_ThenResponse_ShouldContainGreeting() {
    let response = invoke_greet_sync("Carlos");

    assert!(response.ok);
    assert_eq!(response.message, "Hello, Carlos! You've been greeted from Rust!");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn GivenEmptyName_WhenInvokeGreetSyncIsCalled_ThenResponse_ShouldPreserveTemplate() {
    let response = invoke_greet_sync("");

    assert!(response.ok);
    assert_eq!(response.message, "Hello, ! You've been greeted from Rust!");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn GivenNativeAppComponent_WhenItIsBuilt_ThenView_ShouldCompile() {
    let _view = view! { <App /> };
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn GivenAppComponent_WhenRenderedToHtml_ThenMarkup_ShouldContainTitle() {
    let rendered_html = App().to_html();

    assert!(rendered_html.contains("<h1>Welcome to SQL Intelliscan</h1>"));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn GivenEmptyName_WhenShouldSendGreetIsCalled_ThenSubmission_ShouldBeRejected() {
    assert!(!should_send_greet(""));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn GivenNonEmptyName_WhenShouldSendGreetIsCalled_ThenSubmission_ShouldBeAccepted() {
    assert!(should_send_greet("Carlos"));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn GivenEmptyName_WhenGreetMessageIsBuilt_ThenMessage_ShouldBeIgnored() {
    let message = futures::executor::block_on(greet_message(""));

    assert_eq!(message, None);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn GivenNonEmptyName_WhenGreetMessageIsBuilt_ThenMessage_ShouldUseTauriMock() {
    let message = futures::executor::block_on(greet_message("Frontend"));

    assert_eq!(
        message,
        Some("Hello, Frontend! You've been greeted from Rust!".to_string())
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn GivenEmptyName_WhenSyncGreetMessageIsBuilt_ThenMessage_ShouldBeIgnored() {
    let message = greet_message_sync("");

    assert_eq!(message, None);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn GivenNonEmptyName_WhenSyncGreetMessageIsBuilt_ThenMessage_ShouldUseTauriMock() {
    let message = greet_message_sync("Frontend");

    assert_eq!(
        message,
        Some("Hello, Frontend! You've been greeted from Rust!".to_string())
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn GivenNonEmptyName_WhenSpawnGreetRuns_ThenSignal_ShouldReceiveTauriMockMessage() {
    let (message, set_message) = signal(String::new());

    spawn_greet("Frontend".to_string(), set_message);

    assert_eq!(
        message.get_untracked(),
        "Hello, Frontend! You've been greeted from Rust!"
    );
}
