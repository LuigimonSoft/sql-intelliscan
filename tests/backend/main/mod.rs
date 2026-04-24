#![allow(non_snake_case)]

use std::sync::atomic::{AtomicBool, Ordering};
use sql_intelliscan_lib::{
    reset_backend_runner, run_application, set_backend_runner, start_application,
};

#[test]
fn GivenBackendBootstrapper_WhenRunApplicationIsCalled_ThenMain_ShouldDelegateToInjectedRunner() {
    static BACKEND_RUNNER_CALLED: AtomicBool = AtomicBool::new(false);

    fn fake_backend_runner() {
        BACKEND_RUNNER_CALLED.store(true, Ordering::SeqCst);
    }

    BACKEND_RUNNER_CALLED.store(false, Ordering::SeqCst);
    run_application(fake_backend_runner);

    assert!(BACKEND_RUNNER_CALLED.load(Ordering::SeqCst));
}

#[test]
fn GivenBackendRunnerOverride_WhenStartApplicationIsCalled_ThenMain_ShouldUseInjectedBackendRunner() {
    static BACKEND_RUNNER_CALLED: AtomicBool = AtomicBool::new(false);

    fn fake_backend_runner() {
        BACKEND_RUNNER_CALLED.store(true, Ordering::SeqCst);
    }

    BACKEND_RUNNER_CALLED.store(false, Ordering::SeqCst);
    set_backend_runner(fake_backend_runner);

    start_application();

    assert!(BACKEND_RUNNER_CALLED.load(Ordering::SeqCst));

    reset_backend_runner();
}
