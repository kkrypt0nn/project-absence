use std::sync::{LazyLock, Mutex};

pub static LOGGER: LazyLock<Mutex<tangra::Logger>> =
    LazyLock::new(|| Mutex::new(tangra::Logger::new()));

fn parse_event_name(event: String) -> String {
    if !event.is_empty() {
        format!("({})", event)
    } else {
        String::from("")
    }
}

#[allow(dead_code)]
pub fn trace(event: impl Into<String>, message: impl Into<String>) {
    LOGGER.lock().unwrap().println(format!(
        "[$[now:time]] [{}{}{}$[reset]] {}",
        tangra::levels::Level::get_level_color(tangra::levels::Level::TRACE, false),
        tangra::levels::Level::get_level_name(tangra::levels::Level::TRACE).to_lowercase(),
        parse_event_name(event.into()),
        message.into()
    ))
}

#[allow(dead_code)]
pub fn debug(event: impl Into<String>, message: impl Into<String>) {
    LOGGER.lock().unwrap().println(format!(
        "[$[now:time]] [{}{}{}$[reset]] {}",
        tangra::levels::Level::get_level_color(tangra::levels::Level::DEBUG, false),
        tangra::levels::Level::get_level_name(tangra::levels::Level::DEBUG).to_lowercase(),
        parse_event_name(event.into()),
        message.into()
    ))
}

#[allow(dead_code)]
pub fn info(event: impl Into<String>, message: impl Into<String>) {
    LOGGER.lock().unwrap().println(format!(
        "[$[now:time]] [{}{}{}$[reset]] {}",
        tangra::levels::Level::get_level_color(tangra::levels::Level::INFO, false),
        tangra::levels::Level::get_level_name(tangra::levels::Level::INFO).to_lowercase(),
        parse_event_name(event.into()),
        message.into()
    ))
}

#[allow(dead_code)]
pub fn warn(event: impl Into<String>, message: impl Into<String>) {
    LOGGER.lock().unwrap().println(format!(
        "[$[now:time]] [{}{}{}$[reset]] {}",
        tangra::levels::Level::get_level_color(tangra::levels::Level::WARN, false),
        tangra::levels::Level::get_level_name(tangra::levels::Level::WARN).to_lowercase(),
        parse_event_name(event.into()),
        message.into()
    ))
}

pub fn error(event: impl Into<String>, message: impl Into<String>) {
    LOGGER.lock().unwrap().println(format!(
        "[$[now:time]] [{}{}{}$[reset]] {}",
        tangra::levels::Level::get_level_color(tangra::levels::Level::ERROR, false),
        tangra::levels::Level::get_level_name(tangra::levels::Level::ERROR).to_lowercase(),
        parse_event_name(event.into()),
        message.into()
    ))
}

#[allow(dead_code)]
pub fn fatal(event: impl Into<String>, message: impl Into<String>) {
    LOGGER.lock().unwrap().println(format!(
        "[$[now:time]] [{}{}{}$[reset]] {}",
        tangra::levels::Level::get_level_color(tangra::levels::Level::FATAL, false),
        tangra::levels::Level::get_level_name(tangra::levels::Level::FATAL).to_lowercase(),
        parse_event_name(event.into()),
        message.into()
    ))
}

#[allow(dead_code)]
pub fn print(event: impl Into<String>, message: impl Into<String>) {
    LOGGER.lock().unwrap().print(format!(
        "[$[now:time]] [$[effect:bold]$[fg:green]{}$[reset]] {}",
        event.into(),
        message.into()
    ));
}

pub fn println(event: impl Into<String>, message: impl Into<String>) {
    LOGGER.lock().unwrap().println(format!(
        "[$[now:time]] [$[effect:bold]$[fg:green]{}$[reset]] {}",
        event.into(),
        message.into()
    ));
}

#[allow(dead_code)]
pub fn log(level: tangra::levels::Level, message: impl Into<String>) {
    LOGGER.lock().unwrap().log(level, message);
}
