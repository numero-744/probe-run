use probe_rs::{DebugProbeError, Error, Session};

/// This trait is to be applied to results that can lead to warning messages helping the user to
/// fix the issue that raised an error
pub trait Diagnoseable {
    fn diagnose(self) -> Self;
}

impl Diagnoseable for Result<Session, Error> {
    fn diagnose(self) -> Self {
        match &self {
            // Ok should never be diagnosed
            Ok(_) => (),
            // Add some Err(...) branches here to manage the different kinds of errors
            Err(Error::Probe(DebugProbeError::ProbeSpecific(e))) => {
                // FIXME Using `to_string().contains(...)` is a workaround as the concrete type of
                // `e` is not public and therefore does not allow downcasting.
                if e.to_string().contains("JtagNoDeviceConnected") {
                    print_jtag_no_device_connected_info()
                }
            }
            // The other kinds of error are not managed yet, or there is nothing to be done for
            // them
            Err(_) => (),
        }
        self
    }
}

fn print_jtag_no_device_connected_info() {
    eprintln!("Info: Jtag cannot find a connected device.");
    eprintln!("Help:");
    eprintln!("    Check that the debugger is connected to the chip, if so");
    eprintln!("    try using probe-run with option `--connect-under-reset`");
    eprintln!("    or, if using cargo:");
    eprintln!("        cargo run -- --connect-under-reset");
    eprintln!("    If using this flag fixed your issue, this error might");
    eprintln!("    come from the program currently in the chip and using");
    eprintln!("    `--connect-under-reset` is only a workaround.\n");
}
