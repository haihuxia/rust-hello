// Dereference a raw pointer
// Call an unsafe function or method
// Access or modify a mutable static variable
// Implement an unsafe trait

#[cfg(test)]
mod tests {

    #[test]
    fn a() {
        let mut num = 5;

        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }

    fn b() {
        unsafe fn dangerous() {}

        unsafe {
            dangerous();
        }
    }

    #[test]
    fn c() {
        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        // split_at_mut unsafe
        let (a, b) = r.split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    fn d() {
        #[no_mangle]
        pub extern "C" fn call_from_c() {
            println!("Just called a Rust function from C!");
        }
    }

    #[test]
    fn e() {
        static mut COUNTER: u32 = 0;

        fn add_to_count(inc: u32) {
            unsafe {
                COUNTER += inc;
            }
        }

        add_to_count(3);

        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }

    fn f() {
        unsafe trait Foo {
            // methods go here
        }

        unsafe impl Foo for i32 {
            // method implementations go here
        }
    }
}