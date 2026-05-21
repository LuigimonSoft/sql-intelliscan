#![allow(non_snake_case)]

#[cfg(not(target_arch = "wasm32"))]
mod native_render_tests {
    use leptos::prelude::RenderHtml;
    use leptos::prelude::*;
    use sql_intelliscan_ui::app::ConnectionTestFeedback;
    use sql_intelliscan_ui::models::{
        ConnectionTestError, ConnectionTestResult, ConnectionTestStatus,
    };

    fn render_feedback(status: ConnectionTestStatus) -> String {
        view! { <ConnectionTestFeedback status=status /> }.to_html()
    }

    fn successful_result() -> ConnectionTestResult {
        ConnectionTestResult {
            success: true,
            message: "Connection successful.".to_string(),
            database: Some("master".to_string()),
            server_version: Some("Microsoft SQL Server 2022".to_string()),
            latency_ms: Some(42),
        }
    }

    #[test]
    fn GivenSuccessStatus_WhenFeedbackRenders_ThenSafeSuccessDetails_ShouldBeDisplayed() {
        let html = render_feedback(ConnectionTestStatus::Success(successful_result()));

        assert!(html.contains("id=\"connection-feedback\""));
        assert!(html.contains("aria-label=\"Connection test result\""));
        assert!(html.contains("Connection successful."));
        assert!(html.contains("Database"));
        assert!(html.contains("master"));
        assert!(html.contains("Server version"));
        assert!(html.contains("Microsoft SQL Server 2022"));
        assert!(html.contains("Latency"));
        assert!(html.contains("42 ms"));
    }

    #[test]
    fn GivenSuccessStatusWithoutMetadata_WhenFeedbackRenders_ThenMetadata_ShouldBeHidden() {
        let html = render_feedback(ConnectionTestStatus::Success(ConnectionTestResult {
            success: true,
            message: String::new(),
            database: None,
            server_version: None,
            latency_ms: None,
        }));

        assert!(html.contains("Connection successful."));
        assert!(!html.contains("feedback-metadata"));
        assert!(!html.contains("<dt>Database</dt>"));
        assert!(!html.contains("<dt>Server version</dt>"));
        assert!(!html.contains("<dt>Latency</dt>"));
    }

    #[test]
    fn GivenErrorStatus_WhenFeedbackRenders_ThenFriendlyErrorMessage_ShouldBeDisplayed() {
        let html = render_feedback(ConnectionTestStatus::Error(ConnectionTestError {
            code: "AUTHENTICATION_FAILED".to_string(),
            message: "Login failed for user sa; Password=SuperSecret".to_string(),
        }));

        assert!(html.contains("aria-label=\"Connection test result\""));
        assert!(html.contains("Authentication failed. Please verify your credentials."));
        assert!(!html.contains("SuperSecret"));
        assert!(!html.contains("Login failed for user"));
    }

    #[test]
    fn GivenIdleStatus_WhenFeedbackRenders_ThenResultFeedback_ShouldBeHidden() {
        let html = render_feedback(ConnectionTestStatus::Idle);

        assert!(!html.contains("connection-feedback"));
        assert!(!html.contains("Connection successful."));
        assert!(!html.contains("Unable to connect"));
    }

    #[test]
    fn GivenLoadingStatus_WhenFeedbackRenders_ThenStaleResultFeedback_ShouldBeHidden() {
        let html = render_feedback(ConnectionTestStatus::Loading);

        assert!(!html.contains("connection-feedback"));
        assert!(!html.contains("Connection successful."));
        assert!(!html.contains("Unable to connect"));
    }
}

#[test]
fn GivenFeedbackComponentSource_WhenReviewed_ThenPresentationBoundary_ShouldBeRespected() {
    let source = include_str!("../../src/components/connection_test/connection_test_feedback.rs");

    assert!(!source.contains("invoke"));
    assert!(!source.contains("test_connection"));
    assert!(!source.contains("ConnectionTestRequest"));
    assert!(!source.contains("connection_string"));
    assert!(!source.contains("connectionString"));
    assert!(!source.contains("stack"));
    assert!(!source.contains("trace"));
    assert!(!source.contains("backtrace"));
    assert!(!source.contains("println!"));
    assert!(!source.contains("dbg!"));
}
