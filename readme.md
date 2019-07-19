to run
```
git clone https://github.com/FreeMasen/minimal_unescape_test`
cd minimal_unescape_test
cargo run
```
output:
```
failed to escape value "\'\"\\\b\f\n\r\t\v\0"
failed to escape value "\1\00\400\000"
failed to escape value '\'\"\\\b\f\n\r\t\v\0'
failed to escape value '\1\00\400\000'
```

To determine expected values:
```
node
"\'\"\\\b\f\n\r\t\v\0"
"\1\00\400\000"
'\'\"\\\b\f\n\r\t\v\0'
'\1\00\400\000'

```
Node's output:
```
> "\'\"\\\b\f\n\r\t\v\0"
`'"\\\b\f\n\r\t\u000b\u0000`
> "\1\00\400\000"
'\u0001\u0000 0\u0000'
> '\'\"\\\b\f\n\r\t\v\0'
`'"\\\b\f\n\r\t\u000b\u0000`
> '\1\00\400\000'
'\u0001\u0000 0\u0000'
```