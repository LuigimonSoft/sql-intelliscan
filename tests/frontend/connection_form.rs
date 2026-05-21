#![allow(non_snake_case)]

use sql_intelliscan_ui::app::{
    build_connection_test_request, ConnectionFormField, ConnectionFormState,
};

fn field_names(errors: &[sql_intelliscan_ui::app::FieldValidationError]) -> Vec<ConnectionFormField> {
    errors.iter().map(|error| error.field).collect()
}



fn expected_validation_message(field: ConnectionFormField) -> &'static str {
    match field {
        ConnectionFormField::Host => "Enter the SQL Server host.",
        ConnectionFormField::Port => "Enter a port between 1 and 65535.",
        ConnectionFormField::Database => "Enter the database name.",
        ConnectionFormField::Username => "Enter the SQL Server username.",
        ConnectionFormField::Password => "Enter the password.",
        ConnectionFormField::ConnectionTimeout => "Enter a timeout between 1 and 300 seconds.",
        ConnectionFormField::ApplicationName => "Enter an application name or leave it empty.",
    }
}

fn valid_state_with_required_credentials() -> ConnectionFormState {
    ConnectionFormState {
        username: "sa".to_string(),
        password: "StrongPassword123".to_string(),
        ..ConnectionFormState::default()
    }
}

#[test]
fn GivenInvalidFieldValuesOneByOne_WhenRequestIsBuilt_ThenValidationMessages_ShouldMatchField() {
    let scenarios: Vec<(ConnectionFormState, ConnectionFormField)> = vec![
        (
            ConnectionFormState {
                host: "   ".to_string(),
                ..valid_state_with_required_credentials()
            },
            ConnectionFormField::Host,
        ),
        (
            ConnectionFormState {
                port: "0".to_string(),
                ..valid_state_with_required_credentials()
            },
            ConnectionFormField::Port,
        ),
        (
            ConnectionFormState {
                database: " ".to_string(),
                ..valid_state_with_required_credentials()
            },
            ConnectionFormField::Database,
        ),
        (
            ConnectionFormState {
                username: "	".to_string(),
                ..valid_state_with_required_credentials()
            },
            ConnectionFormField::Username,
        ),
        (
            ConnectionFormState {
                password: "
".to_string(),
                ..valid_state_with_required_credentials()
            },
            ConnectionFormField::Password,
        ),
        (
            ConnectionFormState {
                connection_timeout_seconds: "301".to_string(),
                ..valid_state_with_required_credentials()
            },
            ConnectionFormField::ConnectionTimeout,
        ),
        (
            ConnectionFormState {
                application_name: "   ".to_string(),
                ..valid_state_with_required_credentials()
            },
            ConnectionFormField::ApplicationName,
        ),
    ];

    for (state, expected_field) in scenarios {
        let errors = build_connection_test_request(&state).expect_err("request should be invalid");

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, expected_field);
        assert_eq!(
            errors[0].message,
            expected_validation_message(expected_field)
        );
    }
}

#[test]
fn GivenInvalidFieldCombinations_WhenRequestIsBuilt_ThenValidationMessages_ShouldMatchFields() {
    let combined_invalid_state = ConnectionFormState {
        host: " ".to_string(),
        port: "70000".to_string(),
        database: "".to_string(),
        username: "	".to_string(),
        password: " ".to_string(),
        connection_timeout_seconds: "0".to_string(),
        application_name: "   ".to_string(),
        ..ConnectionFormState::default()
    };

    let errors = build_connection_test_request(&combined_invalid_state)
        .expect_err("request should be invalid");

    let expected_fields = [
        ConnectionFormField::Host,
        ConnectionFormField::Port,
        ConnectionFormField::Database,
        ConnectionFormField::Username,
        ConnectionFormField::Password,
        ConnectionFormField::ConnectionTimeout,
        ConnectionFormField::ApplicationName,
    ];

    assert_eq!(errors.len(), expected_fields.len());

    for expected_field in expected_fields {
        let error = errors
            .iter()
            .find(|error| error.field == expected_field)
            .expect("expected validation error for each invalid field");

        assert_eq!(
            error.message,
            expected_validation_message(expected_field)
        );
    }
}
#[test]
fn GivenConnectionFormState_WhenDefaultIsCreated_ThenDefaults_ShouldMatchConnectionRequest() {
    let state = ConnectionFormState::default();

    assert_eq!(state.host, "localhost");
    assert_eq!(state.port, "1433");
    assert_eq!(state.database, "master");
    assert_eq!(state.username, "");
    assert_eq!(state.password, "");
    assert!(state.encrypt);
    assert!(!state.trust_server_certificate);
    assert_eq!(state.connection_timeout_seconds, "30");
    assert_eq!(state.application_name, "SQL Intelliscan");
}

#[test]
fn GivenRequiredFieldsAreEmpty_WhenRequestIsBuilt_ThenValidation_ShouldRejectSubmission() {
    let state = ConnectionFormState {
        host: " ".to_string(),
        database: "".to_string(),
        username: "\t".to_string(),
        password: " ".to_string(),
        ..ConnectionFormState::default()
    };

    let errors = build_connection_test_request(&state).expect_err("request should be invalid");
    let fields = field_names(&errors);

    assert!(fields.contains(&ConnectionFormField::Host));
    assert!(fields.contains(&ConnectionFormField::Database));
    assert!(fields.contains(&ConnectionFormField::Username));
    assert!(fields.contains(&ConnectionFormField::Password));
}

#[test]
fn GivenInvalidPort_WhenRequestIsBuilt_ThenValidation_ShouldReturnUserFriendlyPortError() {
    let state = ConnectionFormState {
        port: "70000".to_string(),
        username: "sa".to_string(),
        password: "StrongPassword123".to_string(),
        ..ConnectionFormState::default()
    };

    let errors = build_connection_test_request(&state).expect_err("request should be invalid");

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].field, ConnectionFormField::Port);
    assert_eq!(errors[0].message, "Enter a port between 1 and 65535.");
}

#[test]
fn GivenInvalidTimeout_WhenRequestIsBuilt_ThenValidation_ShouldRespectBackendLimit() {
    let state = ConnectionFormState {
        username: "sa".to_string(),
        password: "StrongPassword123".to_string(),
        connection_timeout_seconds: "301".to_string(),
        ..ConnectionFormState::default()
    };

    let errors = build_connection_test_request(&state).expect_err("request should be invalid");

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].field, ConnectionFormField::ConnectionTimeout);
    assert_eq!(
        errors[0].message,
        "Enter a timeout between 1 and 300 seconds."
    );
}

#[test]
fn GivenBlankApplicationNameIsProvided_WhenRequestIsBuilt_ThenValidation_ShouldRejectIt() {
    let state = ConnectionFormState {
        username: "sa".to_string(),
        password: "StrongPassword123".to_string(),
        application_name: "   ".to_string(),
        ..ConnectionFormState::default()
    };

    let errors = build_connection_test_request(&state).expect_err("request should be invalid");

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].field, ConnectionFormField::ApplicationName);
}

#[test]
fn GivenApplicationNameIsOmitted_WhenRequestIsBuilt_ThenRequest_ShouldUseNone() {
    let state = ConnectionFormState {
        username: "sa".to_string(),
        password: "StrongPassword123".to_string(),
        application_name: String::new(),
        ..ConnectionFormState::default()
    };

    let request = build_connection_test_request(&state).expect("request should be valid");

    assert_eq!(request.application_name, None);
}

#[test]
fn GivenValidFormState_WhenRequestIsBuilt_ThenRequest_ShouldContainValidatedValues() {
    let state = ConnectionFormState {
        host: " db.example.test ".to_string(),
        port: "1444".to_string(),
        database: " inventory ".to_string(),
        username: " app_user ".to_string(),
        password: " StrongPassword123 ".to_string(),
        encrypt: false,
        trust_server_certificate: true,
        connection_timeout_seconds: "45".to_string(),
        application_name: " SQL Intelliscan Tests ".to_string(),
    };

    let request = build_connection_test_request(&state).expect("request should be valid");

    assert_eq!(request.host, "db.example.test");
    assert_eq!(request.port, 1444);
    assert_eq!(request.database, "inventory");
    assert_eq!(request.username, "app_user");
    assert_eq!(request.password, " StrongPassword123 ");
    assert!(!request.encrypt);
    assert!(request.trust_server_certificate);
    assert_eq!(request.connection_timeout_seconds, 45);
    assert_eq!(
        request.application_name.as_deref(),
        Some("SQL Intelliscan Tests")
    );
}

#[test]
fn GivenConnectionFormSource_WhenReviewed_ThenComponent_ShouldNotUseUnsafeBoundaries() {
    let source = include_str!("../../src/components/connection_test/connection_form.rs");

    assert!(!source.contains("invoke"));
    assert!(!source.contains("localStorage"));
    assert!(!source.contains("sessionStorage"));
    assert!(!source.contains("println!"));
    assert!(!source.contains("dbg!"));
    assert!(!source.contains("console_log"));
    assert!(!source.contains("log::"));
    assert!(!source.contains("connection_string"));
    assert!(!source.contains("connectionString"));
    assert!(source.contains("is_loading.get_untracked()"));
    assert!(source.contains("disabled=move || is_loading.get()"));
}

#[test]
fn GivenConnectionFormSource_WhenReviewed_ThenPassword_ShouldOnlyRenderAsFormInput() {
    let source = include_str!("../../src/components/connection_test/connection_form.rs");

    assert!(source.contains("id=\"connection-password\""));
    assert!(source.contains("name=\"password\""));
    assert!(source.contains("type=move || if show_password.get()"));
    assert!(!source.contains("ConnectionTestFeedback"));
    assert!(!source.contains("connection-status"));
}

#[cfg(not(target_arch = "wasm32"))]
mod native_render_tests {
    use leptos::prelude::*;
    use leptos::prelude::RenderHtml;
    use sql_intelliscan_ui::app::{ConnectionForm, ConnectionTestView};

    #[test]
    fn GivenConnectionForm_WhenRenderedToHtml_ThenPasswordInput_ShouldBeMasked() {
        let html = view! {
            <ConnectionForm on_submit=Callback::new(|_| {}) is_loading=false />
        }
        .to_html();

        assert!(html.contains("id=\"connection-password\""));
        assert!(html.contains("type=\"password\""));
        assert!(!html.contains("StrongPassword123"));
    }

    #[test]
    fn GivenConnectionTestView_WhenRenderedToHtml_ThenConnectionForm_ShouldBePresent() {
        let html = ConnectionTestView().to_html();

        assert!(html.contains("<h1 id=\"connection-title\">SQL Intelliscan</h1>"));
        assert!(html.contains("id=\"connection-form\""));
        assert!(html.contains("id=\"connection-submit\""));
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm_render_tests {
    use leptos::prelude::*;
    use std::sync::{Arc, Mutex};
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::wasm_bindgen_test;
    use web_sys::{Element, Event, HtmlButtonElement, HtmlElement, HtmlInputElement};

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

    #[wasm_bindgen_test]
    fn GivenConnectionForm_WhenMounted_ThenPasswordInput_ShouldBeMasked() {
        console_error_panic_hook::set_once();

        let root = test_root();

        mount_to(
            root.clone()
                .dyn_into::<HtmlElement>()
                .expect("test root should be an html element"),
            || view! { <sql_intelliscan_ui::app::ConnectionForm on_submit=Callback::new(|_| {}) is_loading=false /> },
        )
        .forget();

        let password = root
            .query_selector("#connection-password")
            .expect("selector should not fail")
            .expect("password input should exist")
            .dyn_into::<HtmlInputElement>()
            .expect("password input should be an input");

        assert_eq!(password.type_(), "password");
    }

    #[wasm_bindgen_test]
    fn GivenValidConnectionForm_WhenSubmitted_ThenRequest_ShouldBeEmittedToParent() {
        console_error_panic_hook::set_once();

        let root = test_root();
        let submitted_request = Arc::new(Mutex::new(None));
        let request_sink = Arc::clone(&submitted_request);

        mount_to(
            root.clone()
                .dyn_into::<HtmlElement>()
                .expect("test root should be an html element"),
            move || {
                view! {
                    <sql_intelliscan_ui::app::ConnectionForm on_submit=Callback::new(move |request| {
                        *request_sink.lock().expect("request lock should not be poisoned") = Some(request);
                    }) is_loading=false />
                }
            },
        )
        .forget();

        let username = root
            .query_selector("#connection-username")
            .expect("selector should not fail")
            .expect("username input should exist")
            .dyn_into::<HtmlInputElement>()
            .expect("username input should be an input");
        username.set_value("sa");
        username
            .dispatch_event(&Event::new("input").expect("input event should be created"))
            .expect("input event should dispatch");

        let password = root
            .query_selector("#connection-password")
            .expect("selector should not fail")
            .expect("password input should exist")
            .dyn_into::<HtmlInputElement>()
            .expect("password input should be an input");
        password.set_value("StrongPassword123");
        password
            .dispatch_event(&Event::new("input").expect("input event should be created"))
            .expect("input event should dispatch");

        let button = root
            .query_selector("#connection-submit")
            .expect("selector should not fail")
            .expect("submit button should exist")
            .dyn_into::<HtmlButtonElement>()
            .expect("submit should be a button");
        button.click();

        let request = submitted_request
            .lock()
            .expect("request lock should not be poisoned")
            .clone()
            .expect("request should be submitted");

        assert_eq!(request.host, "localhost");
        assert_eq!(request.port, 1433);
        assert_eq!(request.database, "master");
        assert_eq!(request.username, "sa");
        assert_eq!(request.password, "StrongPassword123");
    }

    #[wasm_bindgen_test]
    fn GivenConnectionForm_WhenLoading_ThenSubmitAction_ShouldBeDisabled() {
        console_error_panic_hook::set_once();

        let root = test_root();

        mount_to(
            root.clone()
                .dyn_into::<HtmlElement>()
                .expect("test root should be an html element"),
            || {
                view! {
                    <sql_intelliscan_ui::app::ConnectionForm
                        on_submit=Callback::new(|_| {})
                        is_loading=true
                    />
                }
            },
        )
        .forget();

        let button = root
            .query_selector("#connection-submit")
            .expect("selector should not fail")
            .expect("submit button should exist")
            .dyn_into::<HtmlButtonElement>()
            .expect("submit should be a button");

        assert!(button.disabled());
    }

    #[wasm_bindgen_test]
    fn GivenInvalidConnectionForm_WhenSubmitted_ThenRequest_ShouldNotBeEmittedToParent() {
        console_error_panic_hook::set_once();

        let root = test_root();
        let submitted_request = Arc::new(Mutex::new(None));
        let request_sink = Arc::clone(&submitted_request);

        mount_to(
            root.clone()
                .dyn_into::<HtmlElement>()
                .expect("test root should be an html element"),
            move || {
                view! {
                    <sql_intelliscan_ui::app::ConnectionForm on_submit=Callback::new(move |request| {
                        *request_sink.lock().expect("request lock should not be poisoned") = Some(request);
                    }) is_loading=false />
                }
            },
        )
        .forget();

        let host = root
            .query_selector("#connection-host")
            .expect("selector should not fail")
            .expect("host input should exist")
            .dyn_into::<HtmlInputElement>()
            .expect("host should be an input");
        host.set_value(" ");
        host.dispatch_event(&Event::new("input").expect("input event should be created"))
            .expect("input event should dispatch");

        let button = root
            .query_selector("#connection-submit")
            .expect("selector should not fail")
            .expect("submit button should exist")
            .dyn_into::<HtmlButtonElement>()
            .expect("submit should be a button");
        button.click();

        assert!(submitted_request
            .lock()
            .expect("request lock should not be poisoned")
            .is_none());
    }
}
