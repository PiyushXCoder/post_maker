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
