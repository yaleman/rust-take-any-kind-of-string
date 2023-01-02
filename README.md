# generic-ish inputs on functions

Example code using #rust tests, seems neat.

This takes a `&'static str` or `String`

```rust
#[cfg(test)]
fn takes_any_kind_of_string(value: impl Into<String>) -> String {
    value.into()
}
```

Testing it:

```rust
#[test]
fn test_with_static_str() {
    let foo = takes_any_kind_of_string("hello world");
    assert_eq!(foo, "hello world".to_string());
}

#[test]
fn test_with_string() {
    let foo = takes_any_kind_of_string("hello world".to_string());
    assert_eq!(foo, "hello world".to_string());
}
```

Found in this article: <https://blog.logrocket.com/build-rust-api-builder-pattern/>
