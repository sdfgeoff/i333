mod application;

fn main() -> std::process::ExitCode {
    i3_common::logging::set_sink(i3_common::logging::stderr_sink);
    std::process::ExitCode::from(application::run() as u8)
}
