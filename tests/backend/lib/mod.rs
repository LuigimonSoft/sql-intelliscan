#![allow(non_snake_case)]

use std::sync::atomic::{AtomicBool, Ordering};

use sql_intelliscan_lib::{build_app, greet, run_with};

#[test]
fn GivenValidName_WhenGreetIsCalled_ThenMessage_ShouldIncludeNameAndBackendOrigin() {
    let result = greet("Carlos");

    assert_eq!(result, "Hello, Carlos! You've been greeted from Rust!");
}

#[test]
fn GivenEmptyName_WhenGreetIsCalled_ThenMessage_ShouldPreserveTemplateWithoutPanicking() {
    let result = greet("");

    assert_eq!(result, "Hello, ! You've been greeted from Rust!");
}

#[test]
fn GivenNameWithUnicodeAndWhitespace_WhenGreetIsCalled_ThenMessage_ShouldPreserveOriginalInput() {
    let result = greet("  José 🚀  ");

    assert_eq!(result, "Hello,   José 🚀  ! You've been greeted from Rust!");
}

#[test]
fn GivenBackendBuilder_WhenAppIsComposed_ThenConfiguration_ShouldBuildWithoutRunningRuntime() {
    let _builder = build_app();
}

#[test]
fn GivenInjectedRunner_WhenRunWithIsCalled_ThenBackend_ShouldDelegateExecutionWithoutLaunchingUi() {
    static RUNNER_CALLED: AtomicBool = AtomicBool::new(false);

    fn fake_runner(_builder: tauri::Builder<tauri::Wry>) {
        RUNNER_CALLED.store(true, Ordering::SeqCst);
    }

    RUNNER_CALLED.store(false, Ordering::SeqCst);
    run_with(build_app, fake_runner);

    assert!(RUNNER_CALLED.load(Ordering::SeqCst));
}
