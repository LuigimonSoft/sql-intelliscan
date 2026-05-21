use crate::models::{ConnectionTestError, ConnectionTestResult, ConnectionTestStatus};
use leptos::prelude::*;

fn success_message_or_default(result: &ConnectionTestResult) -> &str {
    if result.message.trim().is_empty() {
        "Connection successful."
    } else {
        result.message.as_str()
    }
}

fn friendly_error_message(error: &ConnectionTestError) -> &'static str {
    match error.code.as_str() {
        "INVALID_CONFIGURATION" => "Some connection settings are invalid. Please review the form.",
        "AUTHENTICATION_FAILED" => "Authentication failed. Please verify your credentials.",
        "TIMEOUT" => "The connection attempt timed out. Please verify the server is reachable.",
        "CONNECTION_FAILED" => {
            "Unable to connect to the SQL Server instance. Please verify the host and port."
        }
        _ => "An unexpected error occurred while testing the connection.",
    }
}

#[cfg_attr(coverage, allow(dead_code))]
fn metadata_items(result: &ConnectionTestResult) -> Vec<(&'static str, String)> {
    let mut items = Vec::new();

    if let Some(database) = result
        .database
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        items.push(("Database", database.to_string()));
    }

    if let Some(server_version) = result
        .server_version
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        items.push(("Server version", server_version.to_string()));
    }

    if let Some(latency_ms) = result.latency_ms {
        items.push(("Latency", format!("{latency_ms} ms")));
    }

    items
}

#[cfg(not(coverage))]
#[component]
pub fn ConnectionTestFeedback(#[prop(into)] status: Signal<ConnectionTestStatus>) -> impl IntoView {
    view! {
        {move || match status.get() {
            ConnectionTestStatus::Idle | ConnectionTestStatus::Loading => ().into_any(),
            ConnectionTestStatus::Success(result) => {
                let metadata = metadata_items(&result);
                let metadata_view = if metadata.is_empty() {
                    ().into_any()
                } else {
                    view! {
                        <dl class="feedback-metadata">
                            {metadata.into_iter().map(|(label, value)| {
                                view! {
                                    <div class="feedback-row">
                                        <dt>{label}</dt>
                                        <dd>{value}</dd>
                                    </div>
                                }
                            }).collect_view()}
                        </dl>
                    }.into_any()
                };

                view! {
                    <section
                        id="connection-feedback"
                        class="connection-feedback success"
                        aria-live="polite"
                        aria-label="Connection test result"
                    >
                        <strong class="feedback-title">{success_message_or_default(&result).to_string()}</strong>
                        {metadata_view}
                    </section>
                }.into_any()
            }
            ConnectionTestStatus::Error(error) => {
                view! {
                    <section
                        id="connection-feedback"
                        class="connection-feedback error"
                        aria-live="polite"
                        aria-label="Connection test result"
                    >
                        <strong class="feedback-title">{friendly_error_message(&error)}</strong>
                    </section>
                }.into_any()
            }
        }}
    }
}

#[cfg(coverage)]
#[component]
pub fn ConnectionTestFeedback(#[prop(into)] status: Signal<ConnectionTestStatus>) -> impl IntoView {
    view! {
        {move || match status.get() {
            ConnectionTestStatus::Idle | ConnectionTestStatus::Loading => ().into_any(),
            ConnectionTestStatus::Success(result) => {
                let metadata = metadata_items(&result);
                let metadata_view = if metadata.is_empty() {
                    ().into_any()
                } else {
                    view! {
                        <dl class="feedback-metadata">
                            {metadata.into_iter().map(|(label, value)| {
                                view! {
                                    <div class="feedback-row">
                                        <dt>{label}</dt>
                                        <dd>{value}</dd>
                                    </div>
                                }
                            }).collect_view()}
                        </dl>
                    }.into_any()
                };

                view! {
                    <section
                        id="connection-feedback"
                        class="connection-feedback success"
                        aria-live="polite"
                        aria-label="Connection test result"
                    >
                        <strong class="feedback-title">{success_message_or_default(&result).to_string()}</strong>
                        {metadata_view}
                    </section>
                }.into_any()
            }
            ConnectionTestStatus::Error(error) => {
                view! {
                    <section
                        id="connection-feedback"
                        class="connection-feedback error"
                        aria-live="polite"
                        aria-label="Connection test result"
                    >
                        <strong class="feedback-title">{friendly_error_message(&error)}</strong>
                    </section>
                }.into_any()
            }
        }}
    }
}
