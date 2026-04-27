use sql_intelliscan_repository::StaticBackendMetadataRepository;
use sql_intelliscan_services::GreetingService;

pub fn greet_user(name: &str) -> String {
    let service = GreetingService::new(StaticBackendMetadataRepository);

    service.greet(name)
}
