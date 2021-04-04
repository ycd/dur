mod backend;
mod config;
mod dur;

pub use backend::{Backend, Memory};
pub use config::Config;

pub use dur::Dur;

fn main() {
    let mut dur = Dur::new(Memory::new(), None);

    dur.request(1239812419, None);
    dur.request(1239812419, None);
    dur.request(1239812419, None);
    dur.request(1239812419, None);
    dur.request(1239812419, None);
}
