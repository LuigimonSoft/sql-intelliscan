#![allow(non_snake_case)]

use std::sync::atomic::{AtomicBool, Ordering};

#[path = "../../../src-tauri/src/main.rs"]
mod backend_main;

#[test]
fn GivenBackendBootstrapper_WhenRunApplicationIsCalled_ThenMain_ShouldDelegateToInjectedRunner() {
    static BACKEND_RUNNER_CALLED: AtomicBool = AtomicBool::new(false);

    fn fake_backend_runner() {
        BACKEND_RUNNER_CALLED.store(true, Ordering::SeqCst);
    }

    BACKEND_RUNNER_CALLED.store(false, Ordering::SeqCst);
    backend_main::run_application(fake_backend_runner);

    assert!(BACKEND_RUNNER_CALLED.load(Ordering::SeqCst));
}

#[test]
fn GivenBackendRunnerOverride_WhenStartApplicationIsCalled_ThenMain_ShouldUseInjectedBackendRunner() {
    static BACKEND_RUNNER_CALLED: AtomicBool = AtomicBool::new(false);

    fn fake_backend_runner() {
        BACKEND_RUNNER_CALLED.store(true, Ordering::SeqCst);
    }

    BACKEND_RUNNER_CALLED.store(false, Ordering::SeqCst);
    backend_main::set_backend_runner(fake_backend_runner);

    backend_main::start_application();

    assert!(BACKEND_RUNNER_CALLED.load(Ordering::SeqCst));

    backend_main::reset_backend_runner();
}
