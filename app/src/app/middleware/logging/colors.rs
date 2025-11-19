use std::time::SystemTime;

use fern::{
    Dispatch,
    colors::{Color, ColoredLevelConfig},
};

pub(crate) fn default_log_settings() -> Dispatch {
    fern::Dispatch::new()
        .format(|out, message, record| {
            let colors = ColoredLevelConfig::new()
                // use builder methods
                .info(Color::TrueColor {
                    r: 30,
                    g: 128,
                    b: 255,
                })
                .trace(Color::TrueColor {
                    r: 190,
                    g: 240,
                    b: 180,
                })
                .warn(Color::TrueColor {
                    r: 255,
                    g: 141,
                    b: 10,
                });
            out.finish(format_args!(
                "[{} {}] {}",
                humantime::format_rfc3339_millis(SystemTime::now()),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stderr())
}
