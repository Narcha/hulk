use partial::Partial;
use partial_derive::Partial;
use serde::Deserialize;

#[derive(Clone, Debug, Partial, PartialEq)]
#[partial(derive(Debug, Deserialize))]
#[partial(serde(rename_all = "snake_case"))]
#[partial_name = AlmostBar]
struct Bar {
    field3: usize,
}

#[derive(Clone, Debug, Partial, PartialEq)]
#[partial(derive(Default, Debug, Deserialize))]
struct Foo {
    #[partial(serde(skip))]
    field1: usize,
    field2: Bar,
}

#[test]
fn test1() {
    let full_orig = Foo {
        field1: 42,
        field2: Bar { field3: 1337 },
    };
    let partial = PartialFoo::default();

    let mut full = full_orig.clone();
    full.apply_partial(partial);
    assert_eq!(full, full_orig);
}

#[test]
fn test2() {
    let full_orig = Foo {
        field1: 42,
        field2: Bar { field3: 1337 },
    };
    let partial = PartialFoo {
        field1: Some(33),
        field2: None,
    };

    let mut full = full_orig.clone();
    full.apply_partial(partial);
    assert_eq!(
        full,
        Foo {
            field1: 33,
            field2: Bar { field3: 1337 }
        }
    );
}

#[test]
fn test3() {
    let full_orig = Foo {
        field1: 42,
        field2: Bar { field3: 1337 },
    };
    let partial = PartialFoo {
        field1: None,
        field2: Some(PartialBar { field3: None }),
    };

    let mut full = full_orig.clone();
    full.apply_partial(partial);
    assert_eq!(full, full_orig);
}

#[test]
fn test4() {
    let full_orig = Foo {
        field1: 42,
        field2: Bar { field3: 1337 },
    };
    let partial = PartialFoo {
        field1: None,
        field2: Some(PartialBar { field3: Some(0) }),
    };

    let mut full = full_orig.clone();
    full.apply_partial(partial);
    assert_eq!(
        full,
        Foo {
            field1: 42,
            field2: Bar { field3: 0 }
        }
    );
}
