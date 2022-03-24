use std::fmt::Debug;
use std::panic::Location;
use fltk::dialog;

pub trait ResultExt<T, E> {
    fn expect_log(self, msg: &str) -> T;
    fn error_log(&self, msg: &str);
    fn warn_log(&self, msg: &str);
}

impl<T, E: Debug> ResultExt<T, E> for Result<T, E> {
    #[track_caller]
    fn expect_log(self, msg: &str) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                dialog::alert_default(msg);
                error!("{}\n{:?}\n{}", msg, e, Location::caller());
                std::process::exit(1);
            }
        }
    }

    #[track_caller]
    fn error_log(&self, msg: &str) {
        if let Err(e) = self {
            dialog::alert_default(msg);
            error!("{}\n{:?}\n{}", msg, e, Location::caller());
        }
    }

    #[track_caller]
    fn warn_log(&self, msg: &str) {
        if let Err(e) = self {
            dialog::alert_default(msg);
            warn!("{}\n{:?}", msg, e);
        }
    }
}