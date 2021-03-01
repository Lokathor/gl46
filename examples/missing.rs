fn main() {
    let fns = unsafe { gl46::GlFns::load_from(&|_| 1usize as _) };
    let fns = fns.unwrap();
}