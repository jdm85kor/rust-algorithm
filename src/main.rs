#[macro_use]
pub mod search;

fn main() {
    pub use self::search::binary::binary_search;
}
