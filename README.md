# win_lib_loader

 Provides a light wrapper to the Windows API calls for loading DLL libraries, as well as an attribute macro that allows easy definition of the interfaces.

 ## Usage
Using the wrapper:

 ```rust
let test_dll = WinLibrary::load("testdll.dll").unwrap();
type AddFn = fn(i32,i32)->i32;
let add_fn = test_dll.get_function::<AddFn>("add").unwrap();
assert_eq!(8, add_fn(5,3));
 ```

Using the macro:

```rust
#[load_from_dll("testdll.dll")]
pub struct TestDll {
    
    // This will search for "add" in the DLL
    add: fn(a: i32, b: i32) -> i32,

    // This will search for "sub" in the DLL
    // can be useful for searching for decorated names
    #[name("sub")]
    my_sub: fn(a: i32, b: i32) -> i32,
}

fn main(){
    let test_dll = TestDll.new().unwrap();
    assert_eq!(8, test_dll.add(5,3));
    assert_eq!(15, test_dll.my_sub(18,3));
}
```
