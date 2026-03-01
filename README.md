# From Nested Tuple

[![Latest Version]][crates.io]
[![Repo]][GitHub]

[Latest Version]: https://img.shields.io/crates/v/from_nested_tuple.svg
[crates.io]: https://crates.io/crates/from_nested_tuple
[Repo]: https://img.shields.io/badge/github-repo-blue?logo=github
[GitHub]: https://github.com/MasterTemple/from_nested_tuple

_Create structs from left-nested tuples_

---

This is created to easily unnest tuples (from chained `.then` statements) in [`chumsky`](https://github.com/zesterer/chumsky/).

Go from

```rust
some().nested().parser().map(|(((child1, child2), child3), child4)| {
    Parent {
        child1,
        child2,
        child3,
        child4,
    }
})
```

to

```rust
some().nested().parser().from_tuple()
```

## Usage

Import

```rust
use from_nested_tuple::FromTuple;
```

derive (if using on a struct, it is already implemented for built-in tuples)

```rust
#[derive(FromTuple)]
struct WithNamedFields {
    a: char,
    b: char,
    c: char,
}
```

call `.from_tuple()`

```rust
impl WithNamedFields {
    fn parser<'a>() -> impl Parser<'a, &'a str, Self> {
        any().then(any()).then(any()).from_tuple()
    }
}
```

## Exports

- `FromTuple`: for the derive macro (`#[derive(FromTuple)]`) and trait that extends the parser (`.from_tuple()`)
- `FromNestedTuple`: the trait derived by `FromTuple` (automatically imported for tuples 2 to 64)

Note: calling `.from_tuple()` is the same as `.map(FromNestedTuple::from_nested_tuple)`, but requires no extra imports

## Examples

<details>
<summary>Struct with Named Fields</summary>

See: [`examples/named.rs`](./examples/named.rs)

```rust
use chumsky::prelude::*;
use from_nested_tuple::FromTuple;

#[derive(Debug, PartialEq, Eq, FromTuple)]
struct WithNamedFields {
    a: char,
    b: char,
    c: char,
}

impl WithNamedFields {
    fn parser<'a>() -> impl Parser<'a, &'a str, Self> {
        any().then(any()).then(any()).from_tuple()
    }
}

fn main() {
    assert_eq!(
        WithNamedFields::parser().parse("123").unwrap(),
        WithNamedFields {
            a: '1',
            b: '2',
            c: '3'
        }
    );
}
```

which derives

```rust
impl from_nested_tuple::FromNestedTuple for WithNamedFields {
    type Tuple = ((char, char), char);
    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((a, b), c) = tuple;
        Self { a, b, c }
    }
}
```

</details>

<details>
<summary>Struct with Unnamed Fields</summary>

See: [`examples/unnamed.rs`](./examples/unnamed.rs)

```rust
use chumsky::prelude::*;
use from_nested_tuple::FromTuple;

#[derive(Debug, PartialEq, Eq, FromTuple)]
struct WithUnnamedFields(char, char, char);

impl WithUnnamedFields {
    fn parser<'a>() -> impl Parser<'a, &'a str, Self> {
        any().then(any()).then(any()).from_tuple()
    }
}

fn main() {
    assert_eq!(
        WithUnnamedFields::parser().parse("abc").unwrap(),
        WithUnnamedFields('a', 'b', 'c')
    );
}
```

which derives

```rust
impl from_nested_tuple::FromNestedTuple for WithUnnamedFields {
    type Tuple = ((char, char), char);
    fn from_nested_tuple(tuple: Self::Tuple) -> Self {
        let ((field0, field1), field2) = tuple;
        Self(field0, field1, field2)
    }
}
```

</details>

<details>
<summary>Default Tuple</summary>

See: [`examples/tuples.rs`](./examples/tuple.rs)

```rust
use chumsky::prelude::*;
use from_nested_tuple::FromTuple;

fn parser<'a>() -> impl Parser<'a, &'a str, (char, char, char)> {
    any().then(any()).then(any()).from_tuple()
}

fn main() {
    assert_eq!(parser().parse("abc").unwrap(), ('a', 'b', 'c'))
}
```

</details>

## Installation

**Option A**: Modify `Cargo.toml`

```toml
from_nested_tuple = "0.1.1"
```

**Option B**: Run `cargo` command

```bash
cargo add from_nested_tuple
```
