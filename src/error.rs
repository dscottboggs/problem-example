use tray_icon::icon::BadIcon;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("gui")]
    GuiError(#[from] tray_icon::Error),
    #[error("bad icon")]
    BadIcon(#[from] BadIcon),
}

pub type Result<T> = std::result::Result<T, Error>;
