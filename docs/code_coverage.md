```
RUSTFLAGS="-C instrument-coverage" cargo build
$env:RUSTFLAGS="-C instrument-coverage"
set RUSTFLAGS="-C instrument-coverage" 
```

Then you can run like this 
`cargo tarpauline`

or this
`cargo llvm-cov`