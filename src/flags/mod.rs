pub mod domain;

pub const ZERO: usize = 0;

pub fn contains(flags: usize, flag: usize) -> bool {
    (flags & flag) == flag
}

pub fn contains_to_markdown(flags: usize, flag: usize) -> String {
    if contains(flags, flag) {
        String::from("✅")
    } else {
        String::from("❌")
    }
}
