mod backend;
mod config;
mod dur;

pub use backend::Backend;
pub use config::Config;

pub use dur::Dur;

fn main() {
    Dur::new(Backend::Memory, None);
}
