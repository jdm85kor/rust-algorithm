#[macro_use]
pub mod search;

fn main() {
    pub use self::search::binary::binary_search;
    pub use self::search::binary_recursive::binary_search_rec;
    pub use self::search::quick_select::quick_select;
    pub use self::search::exponential::exponential_search;
    pub use self::search::fibonacci::fibonacci_search;
    pub use self::search::interpolation::interpolation_search;
    pub use self::search::jump::jump_search;
    pub use self::search::kth_smallest::kth_smallest;
    pub use self::search::kth_smallest_heap::kth_smallest_heap;
}
