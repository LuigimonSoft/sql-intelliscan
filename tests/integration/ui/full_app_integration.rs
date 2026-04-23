use leptos::prelude::RenderHtml;
use sql_intelliscan_ui::App;

#[test]
fn given_public_application_contract_when_rendering_and_reading_runtime_copy_then_integration_should_validate_complete_basic_experience()
 {
    let rendered_html = App().to_html();

    assert!(rendered_html.contains("<h1>Welcome to SQL Intelliscan</h1>"));
}