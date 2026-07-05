use std::{
    env,
    fmt::Display,
    fs::{File, OpenOptions, create_dir_all},
    io::Write,
    path::PathBuf,
    sync::{LazyLock, Mutex},
};

use crate::APP_NAME;

const LOG_FILE_PATH: &str = "app.log";

static LOG: LazyLock<Mutex<File>> = LazyLock::new(|| {
    let mut path = PathBuf::from(env::var("HOME").unwrap());
    path.push(".local/state");
    path.push(APP_NAME);
    create_dir_all(&path).unwrap();

    path.push(LOG_FILE_PATH);
    Mutex::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap(),
    )
});

pub fn log(msg: impl Display) {
    let mut file = LOG.lock().unwrap();
    writeln!(file, "{}", msg.to_string()).ok();
}
