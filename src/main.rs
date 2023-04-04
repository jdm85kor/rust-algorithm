#[macro_use]
pub mod search;

fn main() {
    pub use self::search::binary::binary_search;
    pub use self::search::binary_recursive::binary_search_rec;
    pub use self::search::quick_select::quick_select;
}
