extern crate ad983x;

mod base;
use base::{destroy, new_ad9833};

#[test]
fn can_create_and_destroy() {
    let dev = new_ad9833(&[]);
    destroy(dev);
}
