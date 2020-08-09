pub mod shader;

mod asset_index;
mod asset_type;
mod format;

pub use asset_index::*;
pub use asset_type::*;
pub use format::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
