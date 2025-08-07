# From Nested Tuple

This is created to easily unnest tuples (from chained `.then` statements) in [`chumsky`](https://github.com/zesterer/chumsky/).

## Usage

Deriving the `FromTuple` trait

```rust
use from_nested_tuple::FromTuple;

#[derive(FromTuple)]
pub struct Parent {
    child1: Child1,
    child2: Child2,
    child3: Child3,
    child4: Child4,
}
```

generates

```rust
impl from_nested_tuple::FromTuple for Parent {
    type Tuple = (((Child1, Child2), Child3), Child4);
    fn from_tuple(tuple: Self::Tuple) -> Self {
        let (((child1, child2), child3), child4) = tuple;
        Self {
            child1,
            child2,
            child3,
            child4,
        }
    }
}
```

so that you can write

```rust
impl Parent {
    pub fn parser() -> chumsky::Parser<'a, &'a str, Self> {
    Child1::parser()
        .then(Child2::parser())
        .then(Child3::parser())
        .then(Child4::parser())
        .map(FromTuple::from_tuple);
    }
}
```

instead of

```rust
impl Parent {
    pub fn parser() -> chumsky::Parser<'a, &'a str, Self> {
    Child1::parser()
        .then(Child2::parser())
        .then(Child3::parser())
        .then(Child4::parser())
        .map(|(((child1, child2), child3), child4)| {
            Self {
                child1,
                child2,
                child3,
                child4,
            }
        });
    }
}
```

because once you get a decent number of fields, it is a mess
