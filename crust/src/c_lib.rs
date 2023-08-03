// c_lib.rs
#[link(name = "sum", kind = "static")]
extern "C" {
    pub fn sum(a: i32, b: i32) -> i32;
}
