/*
    This file is part of Post Maker.
    Post Maker is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.
    Post Maker is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.
    You should have received a copy of the GNU General Public License
    along with Post Maker.  If not, see <https://www.gnu.org/licenses/>
*/

use crate::utils;
use std::{fmt::Debug, panic::Location};

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
