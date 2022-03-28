use std::{fmt::Debug, panic::Location};

use crate::utils;

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
                error!("{}\n{:?}\n{}", msg, e, Location::caller());
                utils::show_program_panic(msg);
                panic!("[panic]");
            }
        }
    }

    #[track_caller]
    fn error_log(&self, msg: &str) {
        if let Err(e) = self {
            error!("{}\n{:?}\n{}", msg, e, Location::caller());
            utils::show_alert(msg);
        }
    }

    #[track_caller]
    fn warn_log(&self, msg: &str) {
        if let Err(e) = self {
            warn!("{}\n{:?}", msg, e);
            utils::show_alert(msg);
        }
    }
}
