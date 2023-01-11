mod error;

use error::Result;

use std::{thread::sleep, time::Duration};

use tray_icon::{icon::Icon, TrayIconBuilder};

static ICON_DATA: &[u8] = include_bytes!("u1F50E.rgba");

fn main() -> Result<()> {
    TrayIconBuilder::new()
        .with_tooltip(
            "Searx Middleman: chooses a new random instance of Searx to submit each search to",
        )
        .with_icon(Icon::from_rgba(ICON_DATA.into(), 1024, 1024)?)
        .build()?;

    sleep(Duration::from_secs(2));
    Ok(())
}
