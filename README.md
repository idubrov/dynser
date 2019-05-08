# Trait object based deserialization in Rust

## Summary

### What?

The goal of this project is to demonstrate technique that could be used to design data models in Rust. The main idea
behind this technique is to define a trait object-based API which then could be used for serializing and deserializing
the data.

API used in this project is located in [src/reflection.rs](src/reflection.rs).

### Why?

There are multiple potential benefits that could be achieved by using this technique:

1. The same "reflection" API could be used for multiple generalized algorithms, in addition to serialization. For
example, validation or running expressions against the data model.
2. Better control over serialization process: error recover, better error reporting, adjustments for discrepancies
between wire format (JSON) and data types. 
3. Compilation times. The generic code is written against trait objects which simplifies work of the compiler. 

### When?

I think, this technique is could be useful when data model is complex, but regular. Complex in the sense that there are
a lot of data types, so benefits are noticeable. Regular in the sense that there are simple rules that these data types
are defined.

For example, for this case of OpenAPI 3.0, these rules are:

1. There are only two primitive data types: `String` and `boolean`
2. Only simple combinations are used (structs, `Option<T>`, `Vec<T>`, `HashMap<String, T>`, etc).

One big limitation of this approach is that it heavily relies on a fact that structs are `Default`-able (could be
created with no data) -- this makes the whole API possible as the API is built around borrowing mutably of struct fields.

That means it makes it harder (not impossible, though) to distinguish between empty string given for a required field
versus not having value for a required field at all.

### How?

There are three components of this technique:

1. Trait object API itself ([src/reflection.rs](src/reflection.rs)). Defines which features could be expressed (which primitive types
are supported, how they could be combined, and so on).
2. Procedural macro to derive implementation of this API for structs ([dynser-derive/src/lib.rs](dynser-derive/src/lib.rs)).
3. Generic algorithm which uses the API to implement certain functionality, for example deserialization ([src/dyndeser.rs](src/dyndeser.rs)).

Finally, there are data types themselves: [src/openapi.rs](src/openapi.rs).

## Performance

One aspect that I was curious about is how that technique would affect performance, given that it uses dynamic dispatch
which would disable certain optimizations which compiler could do otherwise (for example, inlining).

Here are the numbers comparing performance with [`serde`](http://serde.rs) (the test is a just one file being
deserialized). Note that it is not completely fair as I didn't do optimizations for `serde` code, but should be fine for
a ballpark estimation.

There are two runs: one running code as-is, with all the features, and another running code with certain features
disabled. The reason is that certain `serde` functionality makes the performance to drop (might be me not knowing how
to use `serde`), so that `no-flatten` feature flag disables parts of the data model that excercise those paths in `serde`
(which are [untagged unions](https://serde.rs/enum-representations.html#untagged) and [flattening](https://serde.rs/attr-flatten.html)).

`cargo bench` results:

```
test tests::bench_dynamic ... bench:   1,047,609 ns/iter (+/- 129,056)
test tests::bench_serde   ... bench:   1,747,060 ns/iter (+/- 203,389)
```

`cargo bench --features no-flatten` results:

```
test tests::bench_dynamic ... bench:     901,602 ns/iter (+/- 124,475)
test tests::bench_serde   ... bench:     948,064 ns/iter (+/- 108,237)
```

This variant, however, skips large portion of input data (all of the `x-` fields) and also does not handle references
properly (it always deserialize them as a target type, so if target type does not have `$ref` field, it will be ignored).
