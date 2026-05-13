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
pub fn ConnectionForm(on_submit: Callback<ConnectionTestRequest>) -> impl IntoView {
    let defaults = ConnectionFormState::default();
    let (host, set_host) = signal(defaults.host);
    let (port, set_port) = signal(defaults.port);
    let (database, set_database) = signal(defaults.database);
    let (username, set_username) = signal(defaults.username);
    let (password, set_password) = signal(defaults.password);
    let (encrypt, set_encrypt) = signal(defaults.encrypt);
    let (trust_server_certificate, set_trust_server_certificate) =
        signal(defaults.trust_server_certificate);
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

        match build_connection_test_request(&current_state()) {
            Ok(request) => {
                set_errors.set(Vec::new());
                on_submit.run(request);
            }
            Err(validation_errors) => set_errors.set(validation_errors),
        }
    };

    let error_for = move |field| field_error(&errors.get(), field);

    view! {
        <form id="connection-form" class="connection-card" on:submit=submit novalidate>
            <div class="connection-section">
                <span class="connection-section-label">"Server"</span>
            </div>

            <div class="connection-grid">
                <label class="connection-field" for="connection-host">
                    <span>"Host"</span>
                    <input
                        id="connection-host"
                        name="host"
                        type="text"
                        autocomplete="off"
                        spellcheck="false"
                        prop:value=host
                        on:input=move |ev| set_host.set(event_target_value(&ev))
                        aria-invalid=move || error_for(ConnectionFormField::Host).is_some().to_string()
                    />
                    {move || error_for(ConnectionFormField::Host).map(|message| view! { <small class="connection-error">{message}</small> })}
                </label>

                <label class="connection-field" for="connection-port">
                    <span>"Port"</span>
                    <input
                        id="connection-port"
                        name="port"
                        type="number"
                        min="1"
                        max="65535"
                        inputmode="numeric"
                        prop:value=port
                        on:input=move |ev| set_port.set(event_target_value(&ev))
                        aria-invalid=move || error_for(ConnectionFormField::Port).is_some().to_string()
                    />
                    {move || error_for(ConnectionFormField::Port).map(|message| view! { <small class="connection-error">{message}</small> })}
                </label>
            </div>

            <label class="connection-field" for="connection-database">
                <span>"Database"</span>
                <input
                    id="connection-database"
                    name="database"
                    type="text"
                    autocomplete="off"
                    spellcheck="false"
                    prop:value=database
                    on:input=move |ev| set_database.set(event_target_value(&ev))
                    aria-invalid=move || error_for(ConnectionFormField::Database).is_some().to_string()
                />
                {move || error_for(ConnectionFormField::Database).map(|message| view! { <small class="connection-error">{message}</small> })}
            </label>

            <div class="connection-section">
                <span class="connection-section-label">"Authentication"</span>
            </div>

            <label class="connection-field" for="connection-username">
                <span>"Username"</span>
                <input
                    id="connection-username"
                    name="username"
                    type="text"
                    autocomplete="username"
                    spellcheck="false"
                    prop:value=username
                    on:input=move |ev| set_username.set(event_target_value(&ev))
                    aria-invalid=move || error_for(ConnectionFormField::Username).is_some().to_string()
                />
                {move || error_for(ConnectionFormField::Username).map(|message| view! { <small class="connection-error">{message}</small> })}
            </label>

            <label class="connection-field" for="connection-password">
                <span>"Password"</span>
                <input
                    id="connection-password"
                    name="password"
                    type="password"
                    autocomplete="current-password"
                    prop:value=password
                    on:input=move |ev| set_password.set(event_target_value(&ev))
                    aria-invalid=move || error_for(ConnectionFormField::Password).is_some().to_string()
                />
                {move || error_for(ConnectionFormField::Password).map(|message| view! { <small class="connection-error">{message}</small> })}
            </label>

            <div class="connection-section">
                <span class="connection-section-label">"Connection Options"</span>
            </div>

            <div class="connection-grid">
                <label class="connection-field" for="connection-timeout">
                    <span>"Timeout seconds"</span>
                    <input
                        id="connection-timeout"
                        name="connection_timeout_seconds"
                        type="number"
                        min="1"
                        max=MAX_TIMEOUT_SECONDS.to_string()
                        inputmode="numeric"
                        prop:value=connection_timeout_seconds
                        on:input=move |ev| set_connection_timeout_seconds.set(event_target_value(&ev))
                        aria-invalid=move || error_for(ConnectionFormField::ConnectionTimeout).is_some().to_string()
                    />
                    {move || error_for(ConnectionFormField::ConnectionTimeout).map(|message| view! { <small class="connection-error">{message}</small> })}
                </label>

                <label class="connection-field" for="connection-application-name">
                    <span>"Application name"</span>
                    <input
                        id="connection-application-name"
                        name="application_name"
                        type="text"
                        autocomplete="off"
                        spellcheck="false"
                        prop:value=application_name
                        on:input=move |ev| set_application_name.set(event_target_value(&ev))
                        aria-invalid=move || error_for(ConnectionFormField::ApplicationName).is_some().to_string()
                    />
                    {move || error_for(ConnectionFormField::ApplicationName).map(|message| view! { <small class="connection-error">{message}</small> })}
                </label>
            </div>

            <div class="connection-switches">
                <label class="connection-switch" for="connection-encrypt">
                    <input
                        id="connection-encrypt"
                        name="encrypt"
                        type="checkbox"
                        prop:checked=encrypt
                        on:change=move |ev| set_encrypt.set(event_target_checked(&ev))
                    />
                    <span>"Encrypt connection"</span>
                </label>

                <label class="connection-switch" for="connection-trust-certificate">
                    <input
                        id="connection-trust-certificate"
                        name="trust_server_certificate"
                        type="checkbox"
                        prop:checked=trust_server_certificate
                        on:change=move |ev| set_trust_server_certificate.set(event_target_checked(&ev))
                    />
                    <span>"Trust server certificate"</span>
                </label>
            </div>

            <button id="connection-submit" class="connection-submit" type="submit">
                "Test Connection"
            </button>
        </form>
    }
}

#[cfg(coverage)]
#[component]
pub fn ConnectionForm(on_submit: Callback<ConnectionTestRequest>) -> impl IntoView {
    let _ = on_submit;
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
