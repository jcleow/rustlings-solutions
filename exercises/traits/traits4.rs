// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
// Don't change any line other than the marked one.
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a hint.


pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE
// We cannot use dyn here. Instead we need to use &dyn
// A reference to a trait object (&dyn Trait) has a fixed size that is known at compile-time.
// The size of the reference is the same regardless of the underlying type that implements the trait,
//  so Rust can optimize the code based on the size of the reference.
// By contrast, a trait object (dyn Trait) does not have a fixed size because it can refer to any type that implements the trait,
// so Rust cannot optimize the code based on the size of the trait object itself.
fn compare_license_types(software: impl Licensed , software_two: impl Licensed) -> bool {
    software.licensing_info() == software_two.licensing_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(some_software, other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(other_software, some_software));
    }
}
