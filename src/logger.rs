use std::{
    fs::{File, OpenOptions},
    io::Write,
    sync::{LazyLock, Mutex},
};

const LOG_FILE_PATH: &str = "diagnostic.log";

static LOG: LazyLock<Mutex<File>> = LazyLock::new(|| {
    Mutex::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(LOG_FILE_PATH)
            .unwrap(),
    )
});

pub fn log(msg: impl AsRef<str>) {
    let mut file = LOG.lock().unwrap();
    writeln!(file, "{}", msg.as_ref()).ok();
}
