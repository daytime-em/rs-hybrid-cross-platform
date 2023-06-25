/**
 * example.rs: Some basic rust functions and types for use with the ffi bindings. These can be exported
 * or wrapped by the module in `lib.rs`
 */
const FROM_RUST: &str = " from rust";

pub fn just_print() {
    println!("Just print something");
}

/**
 * Returns a new String with " from rust" appended
 */
pub fn append_location(str: &str) -> String {
    let out_str = String::from(str);
    out_str + FROM_RUST
}

/**
 * Invokes a closure you pass in, returning whatever the closure returns 
 */
pub fn invoke_a_closure_cb<F>(str: String, func: F) -> String
where
    F: FnOnce(String) -> String,
{
    func(str)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_invoke_a_closure_cb() {
        use super::invoke_a_closure_cb;

      let start = String::from("Text");

      let out = invoke_a_closure_cb(start, |str| format!("{} And This", str));
      assert_eq!(
        "Text And This", out.as_str(),
        "Strings should be equal"
      )
    }
}
