#![allow(non_snake_case)]

#[cfg(not(target_arch = "wasm32"))]
mod native_render_tests {
    use leptos::prelude::RenderHtml;
    use sql_intelliscan_ui::app::ConnectionTestView;

    #[test]
    fn GivenConnectionTestView_WhenRenderedToHtml_ThenConnectionForm_ShouldBePresent() {
        let html = ConnectionTestView().to_html();

        assert!(html.contains("<h1 id=\"connection-title\">SQL Intelliscan</h1>"));
        assert!(html.contains("id=\"connection-form\""));
        assert!(html.contains("id=\"connection-submit\""));
        assert!(html.contains("id=\"connection-status\""));
    }
}

#[test]
fn GivenConnectionTestViewSource_WhenReviewed_ThenServiceBoundary_ShouldBeRespected() {
    let source = include_str!("../../src/components/connection_test/connection_test_view.rs");

    assert!(source.contains("test_connection(request)"));
    assert!(source.contains("ConnectionTestStatus::Idle"));
    assert!(source.contains("ConnectionTestStatus::Loading"));
    assert!(source.contains("ConnectionTestStatus::Success"));
    assert!(source.contains("ConnectionTestStatus::Error"));
    assert!(source.contains("ConnectionTestFeedback"));
    assert!(source.contains("status.get_untracked()"));
    assert!(source.contains("is_loading"));
    assert!(!source.contains("invoke("));
    assert!(!source.contains("ConnectionTestRequest {"));
    assert!(!source.contains("connection_string"));
    assert!(!source.contains("connectionString"));
    assert!(!source.contains("localStorage"));
    assert!(!source.contains("sessionStorage"));
    assert!(!source.contains("println!"));
    assert!(!source.contains("dbg!"));
}

#[test]
fn GivenConnectionTestViewSource_WhenReviewed_ThenLoadingAndRetryFlow_ShouldBeExplicit() {
    let source = include_str!("../../src/components/connection_test/connection_test_view.rs");

    assert!(source.contains("if matches!(status.get_untracked(), ConnectionTestStatus::Loading)"));
    assert!(source.contains("set_status.set(ConnectionTestStatus::Loading);"));
    assert!(source.contains("spawn_connection_test(request, set_status);"));
    assert!(source.contains("ConnectionTestStatus::Success(result)"));
    assert!(source.contains("ConnectionTestStatus::Error(error)"));
    assert!(source.contains("Signal::derive(move || matches!(status.get(), ConnectionTestStatus::Loading))"));
}

#[test]
fn GivenConnectionTestViewSource_WhenReviewed_ThenSensitiveData_ShouldNotBeRenderedOrPersisted() {
    let source = include_str!("../../src/components/connection_test/connection_test_view.rs");

    assert!(!source.contains("password"));
    assert!(!source.contains("connection_string"));
    assert!(!source.contains("connectionString"));
    assert!(!source.contains("session_storage"));
    assert!(!source.contains("stack_trace"));
    assert!(!source.contains("backtrace"));
}
