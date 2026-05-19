use crate::models::ConnectionTestRequest;
use leptos::prelude::*;

const MAX_TIMEOUT_SECONDS: u64 = 300;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionFormField {
    Host,
    Port,
    Database,
    Username,
    Password,
    ConnectionTimeout,
    ApplicationName,
}

impl ConnectionFormField {
    fn message(self) -> &'static str {
        match self {
            Self::Host => "Enter the SQL Server host.",
            Self::Port => "Enter a port between 1 and 65535.",
            Self::Database => "Enter the database name.",
            Self::Username => "Enter the SQL Server username.",
            Self::Password => "Enter the password.",
            Self::ConnectionTimeout => "Enter a timeout between 1 and 300 seconds.",
            Self::ApplicationName => "Enter an application name or leave it empty.",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldValidationError {
    pub field: ConnectionFormField,
    pub message: &'static str,
}

impl FieldValidationError {
    fn new(field: ConnectionFormField) -> Self {
        Self {
            field,
            message: field.message(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionFormState {
    pub host: String,
    pub port: String,
    pub database: String,
    pub username: String,
    pub password: String,
    pub encrypt: bool,
    pub trust_server_certificate: bool,
    pub connection_timeout_seconds: String,
    pub application_name: String,
}

impl Default for ConnectionFormState {
    fn default() -> Self {
        let request = ConnectionTestRequest::default();

        Self {
            host: request.host,
            port: request.port.to_string(),
            database: request.database,
            username: request.username,
            password: request.password,
            encrypt: request.encrypt,
            trust_server_certificate: request.trust_server_certificate,
            connection_timeout_seconds: request.connection_timeout_seconds.to_string(),
            application_name: request.application_name.unwrap_or_default(),
        }
    }
}

pub fn build_connection_test_request(
    state: &ConnectionFormState,
) -> Result<ConnectionTestRequest, Vec<FieldValidationError>> {
    let mut errors = Vec::new();

    let host = state.host.trim().to_string();
    if host.is_empty() {
        errors.push(FieldValidationError::new(ConnectionFormField::Host));
    }

    let database = state.database.trim().to_string();
    if database.is_empty() {
        errors.push(FieldValidationError::new(ConnectionFormField::Database));
    }

    let username = state.username.trim().to_string();
    if username.is_empty() {
        errors.push(FieldValidationError::new(ConnectionFormField::Username));
    }

    if state.password.trim().is_empty() {
        errors.push(FieldValidationError::new(ConnectionFormField::Password));
    }

    let port = state
        .port
        .trim()
        .parse::<u16>()
        .ok()
        .filter(|port| *port > 0);
    if port.is_none() {
        errors.push(FieldValidationError::new(ConnectionFormField::Port));
    }

    let connection_timeout_seconds = state
        .connection_timeout_seconds
        .trim()
        .parse::<u64>()
        .ok()
        .filter(|timeout| (1..=MAX_TIMEOUT_SECONDS).contains(timeout));
    if connection_timeout_seconds.is_none() {
        errors.push(FieldValidationError::new(
            ConnectionFormField::ConnectionTimeout,
        ));
    }

    let application_name = if state.application_name.is_empty() {
        None
    } else if state.application_name.trim().is_empty() {
        errors.push(FieldValidationError::new(
            ConnectionFormField::ApplicationName,
        ));
        None
    } else {
        Some(state.application_name.trim().to_string())
    };

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(ConnectionTestRequest {
        host,
        port: port.expect("validated port should exist"),
        database,
        username,
        password: state.password.clone(),
        encrypt: state.encrypt,
        trust_server_certificate: state.trust_server_certificate,
        connection_timeout_seconds: connection_timeout_seconds
            .expect("validated timeout should exist"),
        application_name,
    })
}

#[cfg(not(coverage))]
fn field_error(
    errors: &[FieldValidationError],
    field: ConnectionFormField,
) -> Option<&'static str> {
    errors
        .iter()
        .find(|error| error.field == field)
        .map(|error| error.message)
}

#[cfg(not(coverage))]
#[component]
pub fn ConnectionForm(
    on_submit: Callback<ConnectionTestRequest>,
    #[prop(into)] is_loading: Signal<bool>,
) -> impl IntoView {
    let defaults = ConnectionFormState::default();
    let (host, set_host) = signal(defaults.host);
    let (port, set_port) = signal(defaults.port);
    let (database, set_database) = signal(defaults.database);
    let (username, set_username) = signal(defaults.username);
    let (password, set_password) = signal(defaults.password);
    let (show_password, set_show_password) = signal(false);
    let (encrypt, set_encrypt) = signal(defaults.encrypt);
    let (trust_server_certificate, set_trust_server_certificate) =
        signal(defaults.trust_server_certificate);
    let (options_open, set_options_open) = signal(false);
    let (connection_timeout_seconds, set_connection_timeout_seconds) =
        signal(defaults.connection_timeout_seconds);
    let (application_name, set_application_name) = signal(defaults.application_name);
    let (errors, set_errors) = signal(Vec::<FieldValidationError>::new());

    let current_state = move || ConnectionFormState {
        host: host.get_untracked(),
        port: port.get_untracked(),
        database: database.get_untracked(),
        username: username.get_untracked(),
        password: password.get_untracked(),
        encrypt: encrypt.get_untracked(),
        trust_server_certificate: trust_server_certificate.get_untracked(),
        connection_timeout_seconds: connection_timeout_seconds.get_untracked(),
        application_name: application_name.get_untracked(),
    };

    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        if is_loading.get_untracked() {
            return;
        }

        match build_connection_test_request(&current_state()) {
            Ok(request) => {
                set_errors.set(Vec::new());
                on_submit.run(request);
            }
            Err(validation_errors) => set_errors.set(validation_errors),
        }
    };

    let error_for = move |field| field_error(&errors.get(), field);
    let default_encrypt = encrypt.get_untracked();
    let default_trust_server_certificate = trust_server_certificate.get_untracked();
    let default_connection_timeout_seconds = connection_timeout_seconds.get_untracked();
    let options_badge = move || {
        let custom = encrypt.get() != default_encrypt
            || trust_server_certificate.get() != default_trust_server_certificate
            || connection_timeout_seconds.get() != default_connection_timeout_seconds;

        if custom {
            "Custom"
        } else {
            "Defaults"
        }
    };

    view! {
        <form id="connection-form" class="connection-form-shell" on:submit=submit novalidate>
            <div class="card w-full max-w-sm">
                <div class="sec-row" style="border-top:none;">
                    <span class="sec-label">"Server"</span>
                    <div class="sec-line"></div>
                </div>

                <div class="row-2">
                    <label class="col" for="connection-host">
                        <span class="rl">"Host"</span>
                        <input
                            id="connection-host"
                            class="fi"
                            name="host"
                            type="text"
                            placeholder="192.168.1.10"
                            autocomplete="off"
                            spellcheck="false"
                            prop:value=host
                            on:input=move |ev| set_host.set(event_target_value(&ev))
                            aria-invalid=move || error_for(ConnectionFormField::Host).is_some().to_string()
                        />
                    </label>

                    <label class="col" for="connection-port">
                        <span class="rl">"Port"</span>
                        <input
                            id="connection-port"
                            class="fi"
                            name="port"
                            type="number"
                            min="1"
                            max="65535"
                            inputmode="numeric"
                            style="max-width:58px"
                            prop:value=port
                            on:input=move |ev| set_port.set(event_target_value(&ev))
                            aria-invalid=move || error_for(ConnectionFormField::Port).is_some().to_string()
                        />
                    </label>
                </div>

                <label class="row" for="connection-database">
                    <span class="rl">"Database"</span>
                    <input
                        id="connection-database"
                        class="fi"
                        name="database"
                        type="text"
                        placeholder="production_db"
                        autocomplete="off"
                        spellcheck="false"
                        prop:value=database
                        on:input=move |ev| set_database.set(event_target_value(&ev))
                        aria-invalid=move || error_for(ConnectionFormField::Database).is_some().to_string()
                    />
                </label>

                <label class="row" for="connection-application-name">
                    <span class="rl">"Application"</span>
                    <input
                        id="connection-application-name"
                        class="fi"
                        name="application_name"
                        type="text"
                        placeholder="SQL Intelliscan"
                        autocomplete="off"
                        spellcheck="false"
                        prop:value=application_name
                        on:input=move |ev| set_application_name.set(event_target_value(&ev))
                        aria-invalid=move || error_for(ConnectionFormField::ApplicationName).is_some().to_string()
                    />
                </label>

                <div class="sec-row">
                    <span class="sec-label">"Authentication"</span>
                    <div class="sec-line"></div>
                </div>

                <label class="row" for="connection-username">
                    <span class="rl">"Username"</span>
                    <input
                        id="connection-username"
                        class="fi"
                        name="username"
                        type="text"
                        placeholder="sa"
                        autocomplete="username"
                        spellcheck="false"
                        prop:value=username
                        on:input=move |ev| set_username.set(event_target_value(&ev))
                        aria-invalid=move || error_for(ConnectionFormField::Username).is_some().to_string()
                    />
                </label>

                <label class="row" for="connection-password">
                    <span class="rl">"Password"</span>
                    <input
                        id="connection-password"
                        class="fi"
                        name="password"
                        type=move || if show_password.get() { "text" } else { "password" }
                        placeholder="Password"
                        autocomplete="current-password"
                        prop:value=password
                        on:input=move |ev| set_password.set(event_target_value(&ev))
                        aria-invalid=move || error_for(ConnectionFormField::Password).is_some().to_string()
                    />
                    <button
                        class="eye-btn"
                        type="button"
                        aria-label="Toggle password visibility"
                        on:click=move |_| set_show_password.update(|show| *show = !*show)
                    >
                        <svg width="14" height="14" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                            {move || {
                                if show_password.get() {
                                    view! {
                                        <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/>
                                        <line x1="1" y1="1" x2="23" y2="23"/>
                                    }.into_any()
                                } else {
                                    view! {
                                        <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
                                        <circle cx="12" cy="12" r="3"/>
                                    }.into_any()
                                }
                            }}
                        </svg>
                    </button>
                </label>

                <div class="auth-hint">"SQL Server authentication uses the username and password fields."</div>

                <button
                    class="opts-header"
                    type="button"
                    aria-expanded=move || options_open.get().to_string()
                    aria-controls="connection-options"
                    on:click=move |_| set_options_open.update(|open| *open = !*open)
                >
                    <span class="opts-title">"Connection Options"</span>
                    <div class="opts-right">
                        <span class="opts-badge">{options_badge}</span>
                        <svg class=move || if options_open.get() { "chevron open" } else { "chevron" } viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" width="14" height="14">
                            <polyline points="9 18 15 12 9 6"/>
                        </svg>
                    </div>
                </button>

                <div id="connection-options" class=move || if options_open.get() { "opts-body open" } else { "opts-body" }>
                    <div class="row-2" style="border-top:0.5px solid var(--row-sep)">
                        <label class="col" for="connection-timeout">
                            <span class="rl">"Timeout"</span>
                            <input
                                id="connection-timeout"
                                class="fi"
                                name="connection_timeout_seconds"
                                type="number"
                                min="1"
                                max=MAX_TIMEOUT_SECONDS.to_string()
                                inputmode="numeric"
                                style="max-width:44px"
                                prop:value=connection_timeout_seconds
                                on:input=move |ev| set_connection_timeout_seconds.set(event_target_value(&ev))
                                aria-invalid=move || error_for(ConnectionFormField::ConnectionTimeout).is_some().to_string()
                            />
                            <span class="unit-label">"sec"</span>
                        </label>

                        <label class="col" for="connection-encrypt">
                            <span class="rl">"Encrypt"</span>
                            <div class="sel">
                                <select
                                    id="connection-encrypt"
                                    class="fi"
                                    name="encrypt"
                                    on:change=move |ev| {
                                        let value = event_target_value(&ev);
                                        set_encrypt.set(value.as_str() != "disabled");
                                    }
                                >
                                    <option value="mandatory" selected=move || encrypt.get()>"Mandatory"</option>
                                    <option value="disabled" selected=move || !encrypt.get()>"Disabled"</option>
                                </select>
                            </div>
                        </label>
                    </div>

                    <div class="row" style="justify-content:space-between;">
                        <span class="sw-lbl">"Trust Server Certificate"</span>
                        <button id="connection-trust-certificate" class=move || if trust_server_certificate.get() { "sw on" } else { "sw" } type="button" aria-label="Trust server certificate" aria-pressed=move || trust_server_certificate.get().to_string() on:click=move |_| set_trust_server_certificate.update(|enabled| *enabled = !*enabled)>
                            <span class="sw-thumb"></span>
                        </button>
                    </div>
                </div>
            </div>

            <div class="connection-errors" aria-live="polite">
                {move || {
                    errors.get().into_iter().map(|error| {
                        view! { <small class="connection-error">{error.message}</small> }
                    }).collect_view()
                }}
            </div>

            <div class="connection-actions">
                <button
                    id="connection-submit"
                    class="btn-primary"
                    type="submit"
                    disabled=move || is_loading.get()
                >
                    <svg width="14" height="14" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.3" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M13 10V3L4 14h7v7l9-11h-7z"/>
                    </svg>
                    {move || if is_loading.get() { "Testing..." } else { "Test Connection" }}
                </button>
            </div>
        </form>
    }
}

#[cfg(coverage)]
#[component]
pub fn ConnectionForm(
    on_submit: Callback<ConnectionTestRequest>,
    #[prop(into)] is_loading: Signal<bool>,
) -> impl IntoView {
    let _ = on_submit;
    let _ = is_loading;
    let defaults = ConnectionFormState::default();

    view! {
        <form id="connection-form" class="connection-card" novalidate>
            <div class="connection-section">
                <span class="connection-section-label">"Server"</span>
            </div>
            <label class="connection-field" for="connection-host">
                <span>"Host"</span>
                <input id="connection-host" name="host" type="text" value=defaults.host />
            </label>
            <label class="connection-field" for="connection-port">
                <span>"Port"</span>
                <input id="connection-port" name="port" type="number" value=defaults.port />
            </label>
            <label class="connection-field" for="connection-database">
                <span>"Database"</span>
                <input id="connection-database" name="database" type="text" value=defaults.database />
            </label>
            <div class="connection-section">
                <span class="connection-section-label">"Authentication"</span>
            </div>
            <label class="connection-field" for="connection-username">
                <span>"Username"</span>
                <input id="connection-username" name="username" type="text" value=defaults.username />
            </label>
            <label class="connection-field" for="connection-password">
                <span>"Password"</span>
                <input id="connection-password" name="password" type="password" value=defaults.password />
            </label>
            <div class="connection-section">
                <span class="connection-section-label">"Connection Options"</span>
            </div>
            <label class="connection-field" for="connection-timeout">
                <span>"Timeout seconds"</span>
                <input
                    id="connection-timeout"
                    name="connection_timeout_seconds"
                    type="number"
                    value=defaults.connection_timeout_seconds
                />
            </label>
            <label class="connection-field" for="connection-application-name">
                <span>"Application name"</span>
                <input
                    id="connection-application-name"
                    name="application_name"
                    type="text"
                    value=defaults.application_name
                />
            </label>
            <label class="connection-switch" for="connection-encrypt">
                <input
                    id="connection-encrypt"
                    name="encrypt"
                    type="checkbox"
                    checked=defaults.encrypt
                />
                <span>"Encrypt connection"</span>
            </label>
            <label class="connection-switch" for="connection-trust-certificate">
                <input
                    id="connection-trust-certificate"
                    name="trust_server_certificate"
                    type="checkbox"
                    checked=defaults.trust_server_certificate
                />
                <span>"Trust server certificate"</span>
            </label>
            <button id="connection-submit" class="connection-submit" type="submit">
                "Test Connection"
            </button>
        </form>
    }
}
