// This is free and unencumbered software released into the public domain.

//! Checks report contexts both with and without the library's std feature.

#![cfg(all(feature = "sysexits", feature = "error-stack"))]

use error_stack::Report;
#[cfg(feature = "std")]
use error_stack::ResultExt;
use known_errors::sysexits::SysexitsError;

#[test]
fn sysexits_errors_can_be_report_contexts() {
    let report = Report::new(SysexitsError::EX_USAGE);

    assert_eq!(*report.current_context(), SysexitsError::EX_USAGE);
    assert_eq!(report.to_string(), "EX_USAGE");
}

#[cfg(feature = "std")]
#[test]
fn changing_to_sysexits_context_preserves_the_source_error() {
    let error = std::io::Error::from(std::io::ErrorKind::NotFound);
    let context = SysexitsError::from(&error);
    let result: std::io::Result<()> = Err(error);
    let report = result.change_context(context).unwrap_err();

    assert_eq!(*report.current_context(), SysexitsError::EX_NOINPUT);
    assert_eq!(
        report.downcast_ref::<std::io::Error>().unwrap().kind(),
        std::io::ErrorKind::NotFound,
    );
}
