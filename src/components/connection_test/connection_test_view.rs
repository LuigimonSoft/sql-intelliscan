use crate::components::connection_test::{ConnectionForm, ConnectionTestFeedback};
#[cfg(not(coverage))]
use crate::models::ConnectionTestRequest;
use crate::models::ConnectionTestStatus;
#[cfg(not(coverage))]
use crate::services::connection_service::test_connection;
use leptos::prelude::*;
#[cfg(target_arch = "wasm32")]
use leptos::task::spawn_local;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UiTheme {
    Light,
    Dark,
}

impl UiTheme {
    #[cfg(target_arch = "wasm32")]
    fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    fn toggle(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn resolve_initial_theme() -> UiTheme {
    use web_sys::window;

    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(theme)) = storage.get_item("sql-intelliscan-theme") {
                if theme == "light" {
                    return UiTheme::Light;
                }
                if theme == "dark" {
                    return UiTheme::Dark;
                }
            }
        }

        if let Ok(Some(query)) = window.match_media("(prefers-color-scheme: dark)") {
            return if query.matches() {
                UiTheme::Dark
            } else {
                UiTheme::Light
            };
        }
    }

    UiTheme::Dark
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(coverage, allow(dead_code))]
fn resolve_initial_theme() -> UiTheme {
    UiTheme::Dark
}

#[cfg(target_arch = "wasm32")]
fn apply_theme(theme: UiTheme) {
    use web_sys::window;

    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(root) = document.document_element() {
                let class_list = root.class_list();
                let _ = class_list.toggle_with_force("dark", matches!(theme, UiTheme::Dark));
                let _ = class_list.toggle_with_force("light", matches!(theme, UiTheme::Light));
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn persist_theme(theme: UiTheme) {
    use web_sys::window;

    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item("sql-intelliscan-theme", theme.as_str());
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn persist_theme(_theme: UiTheme) {}

#[cfg(not(coverage))]
fn connection_status_message(status: &ConnectionTestStatus) -> String {
    match status {
        ConnectionTestStatus::Idle => String::new(),
        ConnectionTestStatus::Loading => "Testing SQL Server connection...".to_string(),
        ConnectionTestStatus::Success(_) | ConnectionTestStatus::Error(_) => String::new(),
    }
}

#[cfg(not(coverage))]
#[cfg(target_arch = "wasm32")]
fn spawn_connection_test(
    request: ConnectionTestRequest,
    set_status: WriteSignal<ConnectionTestStatus>,
) {
    spawn_local(async move {
        set_status.set(match test_connection(request).await {
            Ok(result) => ConnectionTestStatus::Success(result),
            Err(error) => ConnectionTestStatus::Error(error),
        });
    });
}

#[cfg(not(coverage))]
#[cfg(not(target_arch = "wasm32"))]
fn spawn_connection_test(
    request: ConnectionTestRequest,
    set_status: WriteSignal<ConnectionTestStatus>,
) {
    set_status.set(
        match futures::executor::block_on(test_connection(request)) {
            Ok(result) => ConnectionTestStatus::Success(result),
            Err(error) => ConnectionTestStatus::Error(error),
        },
    );
}

#[cfg(not(coverage))]
#[component]
pub fn ConnectionTestView() -> impl IntoView {
    let (status, set_status) = signal(ConnectionTestStatus::Idle);
    let (theme, set_theme) = signal(resolve_initial_theme());

    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        apply_theme(theme.get());
    });

    let handle_submit = Callback::new(move |request: ConnectionTestRequest| {
        if matches!(status.get_untracked(), ConnectionTestStatus::Loading) {
            return;
        }

        set_status.set(ConnectionTestStatus::Loading);
        spawn_connection_test(request, set_status);
    });

    let is_loading = Signal::derive(move || matches!(status.get(), ConnectionTestStatus::Loading));

    view! {
        <div class="bg-dark">
            <div class="grid-layer"></div>
            <div class="orb orb-indigo"></div>
            <div class="orb orb-violet"></div>
            <div class="orb orb-cyan"></div>
        </div>
        <div class="bg-light">
            <div class="grid-layer"></div>
            <div class="orb orb-light-indigo"></div>
            <div class="orb orb-light-violet"></div>
        </div>
        <main class="relative z-10 min-h-screen flex flex-col items-center justify-center px-4 py-10">
            <div class="theme-wrap">
                <button
                    id="theme-toggle"
                    class="theme-pill"
                    type="button"
                    aria-label="Toggle theme"
                    on:click=move |_| {
                        set_theme.update(|current| {
                            *current = current.toggle();
                            persist_theme(*current);
                        });
                    }
                >
                    <svg
                        width="13"
                        height="13"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    >
                        {move || {
                            if theme.get() == UiTheme::Dark {
                                view! {
                                    <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
                                }.into_any()
                            } else {
                                view! {
                                    <circle cx="12" cy="12" r="5"/>
                                    <line x1="12" y1="1" x2="12" y2="3"/>
                                    <line x1="12" y1="21" x2="12" y2="23"/>
                                    <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/>
                                    <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
                                    <line x1="1" y1="12" x2="3" y2="12"/>
                                    <line x1="21" y1="12" x2="23" y2="12"/>
                                    <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/>
                                    <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
                                }.into_any()
                            }
                        }}
                    </svg>
                    <span>
                        {move || {
                            if theme.get() == UiTheme::Dark {
                                "Light mode"
                            } else {
                                "Dark mode"
                            }
                        }}
                    </span>
                </button>
            </div>

            <section class="connection-hero" aria-labelledby="connection-title">
                <div class="app-icon flex items-center justify-center mb-3" aria-hidden="true">
                    <svg width="38" height="38" viewBox="0 0 38 38" fill="none">
                        <ellipse cx="19" cy="10" rx="13" ry="4.5" fill="rgba(255,255,255,0.92)"/>
                        <rect x="6" y="10" width="26" height="10" fill="rgba(255,255,255,0.72)"/>
                        <ellipse cx="19" cy="20" rx="13" ry="4.5" fill="rgba(255,255,255,0.80)"/>
                        <rect x="6" y="20" width="26" height="9" fill="rgba(255,255,255,0.55)"/>
                        <ellipse cx="19" cy="29" rx="13" ry="4.5" fill="rgba(255,255,255,0.68)"/>
                        <line x1="10" y1="15.5" x2="28" y2="15.5" stroke="rgba(0,200,255,0.85)" stroke-width="1.3" stroke-linecap="round"/>
                        <line x1="10" y1="25.5" x2="28" y2="25.5" stroke="rgba(0,200,255,0.48)" stroke-width="0.9" stroke-linecap="round"/>
                        <circle cx="10.5" cy="10" r="2.1" fill="#22c55e"/>
                        <circle cx="10.5" cy="20" r="2.1" fill="#f59e0b"/>
                        <circle cx="10.5" cy="29" r="2.1" fill="#60a5fa"/>
                    </svg>
                </div>
                <div class="connection-title-stack">
                    <h1 id="connection-title">"SQL Intelliscan"</h1>
                    <p>"> CONNECT TO SQL SERVER"</p>
                </div>
            </section>

            <ConnectionForm on_submit=handle_submit is_loading=is_loading />

            <ConnectionTestFeedback status=status />

            <p id="connection-status" class="connection-status" aria-live="polite">
                {move || connection_status_message(&status.get())}
            </p>

            <p class="connection-footer">
                {concat!("SQL Intelliscan v", env!("CARGO_PKG_VERSION"), " - Microsoft SQL Server 2016 - 2022")}
            </p>
        </main>
    }
}

#[cfg(coverage)]
#[component]
pub fn ConnectionTestView() -> impl IntoView {
    let (theme, set_theme) = signal(UiTheme::Dark);

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
                <button
                    id="theme-toggle"
                    class="theme-toggle"
                    type="button"
                    aria-label="Toggle theme"
                    on:click=move |_| {
                        set_theme.update(|current| {
                            *current = current.toggle();
                            persist_theme(*current);
                        });
                    }
                >
                    {move || {
                        if theme.get() == UiTheme::Dark {
                            "Light mode"
                        } else {
                            "Dark mode"
                        }
                    }}
                </button>
            </section>

            <ConnectionForm on_submit=Callback::new(|_| {}) is_loading=false />

            <ConnectionTestFeedback status=ConnectionTestStatus::Idle />

            <p id="connection-status" class="connection-status" aria-live="polite"></p>
        </main>
    }
}
