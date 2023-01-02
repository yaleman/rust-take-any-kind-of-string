/// This takes either a `&'static str` or a `String` and returns a `String`.
///```
///use rust_take_any_kind_of_string::takes_any_kind_of_string;
///
///fn test_with_static_str() {
///    let foo = takes_any_kind_of_string("hello world");
///    assert_eq!(foo, "hello world".to_string());
///
///    let bar = takes_any_kind_of_string("hello world".to_string());
///    assert_eq!(bar, "hello world".to_string());
///}
#[allow(dead_code)]
pub fn takes_any_kind_of_string(value: impl Into<String>) -> String {
    value.into()
}
