use crate::Error;
use msgbox;

/// Wraps a result and shows a message box if its an error
pub fn catch<F>(f: F)
    where F: FnOnce() -> Result<(), Error> {
    let result = f();
    match result {
        Ok(_) => {},
        Err(error) => {
            println!("Error: {:#}", error);
            msgbox::create(
                "Error",
                &format!("{:#}", error),
                msgbox::IconType::Error,
            );
        },
    }
}
