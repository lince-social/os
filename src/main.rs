mod features;
use features::{anicca::anicca, dotfiles::dotfiles};

fn main() {
    let _ = anicca();
    let _ = dotfiles();
}
