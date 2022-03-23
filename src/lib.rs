use std::sync::atomic::AtomicUsize; // DEBUG

/* evil globals */
//#[no_mangle] pub static DEBUG: AtomicUsize = AtomicUsize::new(0);
pub static DEBUG: AtomicUsize = AtomicUsize::new(0);
pub const CONF: &str = "etc/conf.ini";

#[cfg(test)]
mod lib_tests {
    use super::*;
    use std::sync::atomic::Ordering;
    #[test]
    fn verify_debug_exists() {
        assert_eq!(DEBUG.load(Ordering::Relaxed), 0_usize);
    }
} //tests
