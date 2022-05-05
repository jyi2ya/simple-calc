## simple-calc

a calculator like `bc(1)`

### features

* operators
  + unary `+` and `-`
  + binary `+` `-` `*` `/` and `%`
* bracket support

### build

```shell
$ cargo build --release
```

### bugs

too long expressions may cause stack overflow
