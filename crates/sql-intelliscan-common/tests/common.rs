#![allow(non_snake_case)]

use sql_intelliscan_common::{backend_origin, BACKEND_ORIGIN};

#[test]
fn GivenCommonLayer_WhenBackendOriginIsRequested_ThenValue_ShouldExposeSharedBackendOrigin() {
    assert_eq!(backend_origin(), BACKEND_ORIGIN);
    assert_eq!(BACKEND_ORIGIN, "Rust");
}
