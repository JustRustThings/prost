include!(concat!(env!("OUT_DIR"), "/wrapped_fields.rs"));

#[test]
/// - Confirm `Foo::bar` and `foo::OneofField::BoxQux` is boxed by creating an instance
/// - `Foo::boxed_bar_list` should not be boxed as it is a `Vec`, therefore it is already heap allocated
fn test_boxed_field() {
    use self::foo::OneofField;
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    let foo = Foo {
        bar: Some(Box::new(Bar {})),
        oneof_field: Some(OneofField::BoxQux(Box::new(Bar {}))),
        boxed_bar_list: Vec::from([Bar {}]),
        opt_bar: Some(Box::new(Bar {})),
        big_bar: Some(Box::new(BigBar {})),
    };
    let _ = Foo {
        oneof_field: Some(OneofField::Baz("hello".into())),
        ..foo
    };
}

#[test]
/// - Confirm `Baz::bar` and `baz::OneofField::BoxQux` is boxed by creating an instance
/// - `Baz::boxed_bar_list` should not be boxed as it is a `Vec`, therefore it is already heap allocated
fn test_arc_field() {
    use self::baz::OneofField;
    use alloc::sync::Arc;
    use alloc::vec::Vec;
    let baz = Baz {
        bar: Some(Arc::new(Bar {})),
        oneof_field: Some(OneofField::BoxQux(Arc::new(Bar {}))),
        boxed_bar_list: Vec::from([Bar {}]),
        opt_bar: Some(Arc::new(Bar {})),
        big_baz: Some(Arc::new(BigBaz {})),
    };
    let _ = Baz {
        oneof_field: Some(OneofField::Baz("hello".into())),
        ..baz
    };
}
