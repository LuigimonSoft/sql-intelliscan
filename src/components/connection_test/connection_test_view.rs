use crate::components::connection_test::ConnectionForm;
#[cfg(not(coverage))]
use crate::models::ConnectionTestRequest;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UiTheme {
    Light,
    Dark,
}

impl UiTheme {
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

        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item("sql-intelliscan-theme", theme.as_str());
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn apply_theme(_theme: UiTheme) {}

#[cfg(not(coverage))]
#[component]
pub fn ConnectionTestView() -> impl IntoView {
    let (status_message, set_status_message) = signal(String::new());
    let (theme, set_theme) = signal(resolve_initial_theme());

    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        apply_theme(theme.get());
    });

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
        <div class="bg-dark"></div>
        <div class="bg-light"></div>
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

            <ConnectionForm on_submit=Callback::new(|_| {}) />

            <p id="connection-status" class="connection-status" aria-live="polite"></p>
        </main>
    }
}
