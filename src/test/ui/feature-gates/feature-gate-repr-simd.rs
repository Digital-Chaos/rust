#[repr(simd)] //~ error: SIMD types are experimental
struct Foo(u64, u64);

#[repr(C)] //~ ERROR conflicting representation hints
#[repr(simd)] //~ error: SIMD types are experimental
struct Bar(u64, u64);

fn main() {}
