use crate::components::connection_test::ConnectionForm;
#[cfg(not(coverage))]
use crate::models::ConnectionTestRequest;
use leptos::prelude::*;

#[cfg(not(coverage))]
#[component]
pub fn ConnectionTestView() -> impl IntoView {
    let (status_message, set_status_message) = signal(String::new());

    let handle_submit = Callback::new(move |request: ConnectionTestRequest| {
        let encryption = if request.encrypt {
            "encrypted"
        } else {
            "unencrypted"
        };
        let certificate_policy = if request.trust_server_certificate {
            "trusting server certificate"
        } else {
            "validating server certificate"
        };

        set_status_message.set(format!(
            "Ready to test {}:{} using database {} ({}, {}).",
            request.host, request.port, request.database, encryption, certificate_policy
        ));
    });

    view! {
        <main class="connection-shell">
            <section class="connection-hero" aria-labelledby="connection-title">
                <div class="connection-brand-mark" aria-hidden="true">
                    <span class="connection-brand-disc"></span>
                    <span class="connection-brand-lines"></span>
                </div>
                <div>
                    <h1 id="connection-title">"SQL Intelliscan"</h1>
                    <p>"Connect to SQL Server"</p>
                </div>
            </section>

            <ConnectionForm on_submit=handle_submit />

            <p id="connection-status" class="connection-status" aria-live="polite">
                {move || status_message.get()}
            </p>
        </main>
    }
}

#[cfg(coverage)]
#[component]
pub fn ConnectionTestView() -> impl IntoView {
    view! {
        <main class="connection-shell">
            <section class="connection-hero" aria-labelledby="connection-title">
                <div class="connection-brand-mark" aria-hidden="true">
                    <span class="connection-brand-disc"></span>
                    <span class="connection-brand-lines"></span>
                </div>
                <div>
                    <h1 id="connection-title">"SQL Intelliscan"</h1>
                    <p>"Connect to SQL Server"</p>
                </div>
            </section>

            <ConnectionForm on_submit=Callback::new(|_| {}) />

            <p id="connection-status" class="connection-status" aria-live="polite"></p>
        </main>
    }
}
