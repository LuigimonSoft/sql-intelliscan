use crate::components::connection_test::ConnectionForm;
use crate::models::{ConnectionTestRequest, ConnectionTestStatus};
use crate::services::connection_service::test_connection;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn ConnectionTestView() -> impl IntoView {
    let (status, set_status) = signal(ConnectionTestStatus::Idle);

    let handle_submit = Callback::new(move |request: ConnectionTestRequest| {
        set_status.set(ConnectionTestStatus::Loading);

        spawn_local({
            let set_status = set_status;
            async move {
                match test_connection(request).await {
                    Ok(result) => set_status.set(ConnectionTestStatus::Success(result)),
                    Err(error) => set_status.set(ConnectionTestStatus::Error(error)),
                }
            }
        });
    });

    let status_message = move || match status.get() {
        ConnectionTestStatus::Idle => {
            "Enter SQL Server details and run a connection test.".to_string()
        }
        ConnectionTestStatus::Loading => "Testing SQL Server connection...".to_string(),
        ConnectionTestStatus::Success(_) => "Connection test completed successfully.".to_string(),
        ConnectionTestStatus::Error(_) => {
            "Connection test failed. Review details and try again.".to_string()
        }
    };

    view! {
        <section class="connection-test-view" aria-labelledby="connection-test-title">
            <h2 id="connection-test-title">"SQL Server Connection"</h2>
            <ConnectionForm on_submit=handle_submit />
            <p class="connection-status" aria-live="polite">{status_message}</p>
        </section>
    }
}
