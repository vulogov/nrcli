extern crate log;
use env_logger::Env;

pub mod cmd;

fn main() {
    let env = Env::default()
        .filter_or("NRCLI_LOG_LEVEL", "trace")
        .write_style_or("NRCLI_LOG_STYLE", "always");

    env_logger::init_from_env(env);
    log::trace!("NRCLI main() function is reached");
    cmd::init()
}
