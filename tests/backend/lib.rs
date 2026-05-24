#![allow(non_snake_case)]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

use sql_intelliscan_lib::{
    build_app, greet, reset_run_hooks, run, run_startup_with, run_with, set_run_hooks,
    StartupLogEvent,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StartupStep {
    LoggingInitialized,
    StartupLogged(StartupLogEvent),
    BuilderCreated,
    RunnerCalled,
}

static STARTUP_STEPS: OnceLock<Mutex<Vec<StartupStep>>> = OnceLock::new();

fn startup_steps() -> &'static Mutex<Vec<StartupStep>> {
    STARTUP_STEPS.get_or_init(|| Mutex::new(Vec::new()))
}

fn record_startup_step(step: StartupStep) {
    startup_steps()
        .lock()
        .expect("startup steps lock poisoned")
        .push(step);
}

#[test]
fn GivenValidName_WhenGreetIsCalled_ThenMessage_ShouldIncludeNameAndBackendOrigin() {
    let result = greet("Carlos").expect("greeting should resolve");

    assert_eq!(result, "Hello, Carlos! You've been greeted from Rust!");
}

#[test]
fn GivenEmptyName_WhenGreetIsCalled_ThenMessage_ShouldPreserveTemplateWithoutPanicking() {
    let result = greet("").expect("greeting should resolve");

    assert_eq!(result, "Hello, ! You've been greeted from Rust!");
}

#[test]
fn GivenNameWithUnicodeAndWhitespace_WhenGreetIsCalled_ThenMessage_ShouldPreserveOriginalInput() {
    let result = greet("  José 🚀  ").expect("greeting should resolve");

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

#[test]
fn GivenRunHooksOverride_WhenRunIsCalled_ThenBackend_ShouldUseInjectedBuilderAndRunner() {
    static RUNNER_CALLED: AtomicBool = AtomicBool::new(false);

    fn fake_builder() -> tauri::Builder<tauri::Wry> {
        build_app()
    }

    fn fake_runner(_builder: tauri::Builder<tauri::Wry>) {
        RUNNER_CALLED.store(true, Ordering::SeqCst);
    }

    RUNNER_CALLED.store(false, Ordering::SeqCst);
    set_run_hooks(fake_builder, fake_runner);

    run();

    assert!(RUNNER_CALLED.load(Ordering::SeqCst));

    reset_run_hooks();
}

#[test]
fn GivenBackendStartup_WhenRunStartupExecutes_ThenLogging_ShouldInitializeBeforeAppWiring() {
    fn fake_logging_initializer() -> Result<(), sql_intelliscan_lib::LoggingInitError> {
        record_startup_step(StartupStep::LoggingInitialized);

        Ok(())
    }

    fn fake_builder() -> tauri::Builder<tauri::Wry> {
        record_startup_step(StartupStep::BuilderCreated);

        tauri::Builder::default()
    }

    fn fake_runner(_builder: tauri::Builder<tauri::Wry>) {
        record_startup_step(StartupStep::RunnerCalled);
    }

    fn fake_startup_logger(event: StartupLogEvent) {
        record_startup_step(StartupStep::StartupLogged(event));
    }

    startup_steps()
        .lock()
        .expect("startup steps lock poisoned")
        .clear();

    run_startup_with(
        fake_logging_initializer,
        fake_builder,
        fake_runner,
        fake_startup_logger,
    )
    .expect("startup should complete");

    let steps = startup_steps()
        .lock()
        .expect("startup steps lock poisoned")
        .clone();

    assert_eq!(
        steps,
        vec![
            StartupStep::LoggingInitialized,
            StartupStep::StartupLogged(StartupLogEvent::ApplicationStartupStarted),
            StartupStep::StartupLogged(StartupLogEvent::LoggingInitialized),
            StartupStep::StartupLogged(StartupLogEvent::ApplicationStateBuildStarted),
            StartupStep::BuilderCreated,
            StartupStep::StartupLogged(StartupLogEvent::TauriApplicationStarting),
            StartupStep::RunnerCalled,
        ]
    );
}
