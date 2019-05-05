# Trait-object based deserialization in Rust

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