#[cfg(test)]
mod tests {
    use win_lib_loader::load_from_dll;

    #[load_from_dll("testdll.dll")]
    pub struct TestDll {
        add: fn(a: i32, b: i32) -> i32,
        #[name("sub")]
        my_sub: fn(a: i32, b: i32) -> i32,
    }

    #[test]
    fn test_dll() {
        println!("{}", std::env::current_dir().unwrap().display());
        let test_dll = TestDll::new().unwrap();
        assert_eq!(11, unsafe { test_dll.add(5, 6) });
        assert_eq!(15, unsafe { test_dll.my_sub(18, 3) });
    }
}
