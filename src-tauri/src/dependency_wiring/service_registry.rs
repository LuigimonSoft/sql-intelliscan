use sql_intelliscan_services::greet as greet_from_services;

pub fn greet_user(name: &str) -> String {
    greet_from_services(name)
}
