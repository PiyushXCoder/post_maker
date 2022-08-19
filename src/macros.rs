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

#[macro_export]
macro_rules! rw_read {
    ($var:expr) => {
        $var.read().expect_log("Program got panic!")
    };
}

#[macro_export]
macro_rules! rw_write {
    ($var:expr) => {
        $var.write().expect_log("Program got panic!")
    };
}
