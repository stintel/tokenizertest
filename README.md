Can someone explain why the result for input ["foo", "bar"] differs?

```
➜ python tokenizertest.py
None of PyTorch, TensorFlow >= 2.0, or Flax have been found. Models won't be available and only tokenizers, configuration and file/data utilities can be used.
input: 'foo'
output: [101, 29379, 102]

input: 'foo bar'
output: [101, 29379, 3347, 102]

input: ['foo', 'bar']
output: [101, 29379, 102, 3347, 102]
```

```
➜ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/tokenizertest`
input: "foo"
output: [101, 29379, 102]

input: "foo bar"
output: [101, 29379, 3347, 102]

input: ["foo", "bar"]
output: [101, 29379, 3347, 102]
```
