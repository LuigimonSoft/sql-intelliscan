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
use web_sys::{Element, Event, HtmlButtonElement, HtmlElement, HtmlInputElement};

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(target_arch = "wasm32")]
async fn flush_ui_updates() {
    let _ = JsFuture::from(js_sys::Promise::resolve(&JsValue::NULL)).await;
    let _ = JsFuture::from(js_sys::Promise::resolve(&JsValue::NULL)).await;
    let _ = JsFuture::from(js_sys::Promise::resolve(&JsValue::NULL)).await;
}

#[cfg(target_arch = "wasm32")]
fn test_root() -> Element {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .expect("document should be available");
    let root = document
        .create_element("section")
        .expect("test root should be created");
    document
        .body()
        .expect("document body should exist")
        .append_child(&root)
        .expect("test root should be attached");
    root
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn GivenAppComponent_WhenMounted_ThenH1_ShouldRenderExpectedTitle() {
    console_error_panic_hook::set_once();

    let root = test_root();
    mount_to(
        root.clone()
            .dyn_into::<HtmlElement>()
            .expect("test root should be an html element"),
        || view! { <App /> },
    )
    .forget();

    let title = root
        .query_selector("h1")
        .expect("selector should not fail");

    assert!(title.is_some(), "App should render an <h1> element");
    assert_eq!(
        title.unwrap().text_content().unwrap(),
        "SQL Intelliscan",
        "The <h1> element should contain the correct text"
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test(async)]
async fn GivenAppComponent_WhenConnectionFormIsSubmitted_ThenStatus_ShouldRenderSafeRequestSummary()
{
    console_error_panic_hook::set_once();

    let root = test_root();

    mount_to(
        root.clone()
            .dyn_into::<HtmlElement>()
            .expect("test root should be an html element"),
        || view! { <App /> },
    )
    .forget();

    let username = root
        .query_selector("#connection-username")
        .expect("selector should not fail")
        .expect("connection username input should exist")
        .dyn_into::<HtmlInputElement>()
        .expect("connection username should be an input element");
    let password = root
        .query_selector("#connection-password")
        .expect("selector should not fail")
        .expect("connection password input should exist")
        .dyn_into::<HtmlInputElement>()
        .expect("connection password should be an input element");
    let button = root
        .query_selector("#connection-submit")
        .expect("selector should not fail")
        .expect("connection submit button should exist")
        .dyn_into::<HtmlButtonElement>()
        .expect("connection submit should be a button element");

    username.set_value("sa");
    username
        .dispatch_event(&Event::new("input").expect("input event should be created"))
        .expect("input event should dispatch");
    password.set_value("StrongPassword123");
    password
        .dispatch_event(&Event::new("input").expect("input event should be created"))
        .expect("input event should dispatch");
    button.click();
    flush_ui_updates().await;

    let feedback = root
        .query_selector("#connection-feedback")
        .expect("selector should not fail")
        .and_then(|element| element.text_content())
        .expect("feedback should expose text");

    assert!(feedback.contains("An unexpected error occurred while testing the connection."));
    assert!(!feedback.contains("Tauri backend is not available."));
    assert!(!feedback.contains("StrongPassword123"));
    assert_eq!(password.type_(), "password");
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test(async)]
async fn GivenNoStoredTheme_WhenAppIsMounted_ThenTheme_ShouldNotBePersistedUntilToggle() {
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("window should be available");
    let storage = window
        .local_storage()
        .expect("local storage access should not fail")
        .expect("local storage should be available");
    storage
        .remove_item("sql-intelliscan-theme")
        .expect("theme preference should be cleared");

    let root = test_root();
    mount_to(
        root.clone()
            .dyn_into::<HtmlElement>()
            .expect("test root should be an html element"),
        || view! { <App /> },
    )
    .forget();
    flush_ui_updates().await;

    assert_eq!(
        storage
            .get_item("sql-intelliscan-theme")
            .expect("theme preference read should not fail"),
        None
    );

    root.query_selector("#theme-toggle")
        .expect("selector should not fail")
        .expect("theme toggle should exist")
        .dyn_into::<HtmlButtonElement>()
        .expect("theme toggle should be a button")
        .click();
    flush_ui_updates().await;

    assert!(
        storage
            .get_item("sql-intelliscan-theme")
            .expect("theme preference read should not fail")
            .is_some()
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test(async)]
async fn GivenStoredLightTheme_WhenAppIsMounted_ThenTheme_ShouldApplyStoredPreference() {
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("window should be available");
    let storage = window
        .local_storage()
        .expect("local storage access should not fail")
        .expect("local storage should be available");
    storage
        .set_item("sql-intelliscan-theme", "light")
        .expect("theme preference should be stored");

    let root = test_root();
    mount_to(
        root.clone()
            .dyn_into::<HtmlElement>()
            .expect("test root should be an html element"),
        || view! { <App /> },
    )
    .forget();
    flush_ui_updates().await;

    let class_list = window
        .document()
        .and_then(|document| document.document_element())
        .expect("document element should exist")
        .class_list();

    assert!(class_list.contains("light"));
    assert!(!class_list.contains("dark"));
}


#[cfg(not(target_arch = "wasm32"))]
#[test]
fn GivenConnectionFormValues_WhenRequestIsBuilt_ThenRequest_ShouldContainSubmittedValues() {
    let state = sql_intelliscan_ui::app::ConnectionFormState {
        host: "localhost".to_string(),
        port: "1444".to_string(),
        database: "inventory".to_string(),
        username: "sa".to_string(),
        password: "StrongPassword123".to_string(),
        encrypt: false,
        trust_server_certificate: true,
        connection_timeout_seconds: "45".to_string(),
        application_name: "SQL Intelliscan Integration Test".to_string(),
    };

    let request = sql_intelliscan_ui::app::build_connection_test_request(&state)
        .expect("form values should build a valid connection request");

    assert_eq!(request.host, "localhost");
    assert_eq!(request.port, 1444);
    assert_eq!(request.database, "inventory");
    assert_eq!(request.username, "sa");
    assert_eq!(request.password, "StrongPassword123");
    assert!(!request.encrypt);
    assert!(request.trust_server_certificate);
    assert_eq!(request.connection_timeout_seconds, 45);
    assert_eq!(
        request.application_name.as_deref(),
        Some("SQL Intelliscan Integration Test")
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

    assert!(rendered_html.contains("<h1 id=\"connection-title\">SQL Intelliscan</h1>"));
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
fn GivenWhitespaceName_WhenShouldSendGreetIsCalled_ThenSubmission_ShouldBeRejected() {
    assert!(!should_send_greet("   \t\n"));
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
fn GivenNameWithLeadingAndTrailingWhitespace_WhenSyncGreetMessageIsBuilt_ThenMessage_ShouldUseTrimmedValue(
) {
    let message = greet_message_sync("  Frontend  ");

    assert_eq!(
        message,
        Some("Hello, Frontend! You've been greeted from Rust!".to_string())
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn GivenNameWithLeadingAndTrailingWhitespace_WhenGreetMessageIsBuilt_ThenMessage_ShouldUseTrimmedValue()
{
    let message = futures::executor::block_on(greet_message("  Frontend  "));

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
