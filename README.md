# File Finder
### A "file finding" utility
To build from source and include native optimizations (specific to your CPU), do: ```RUSTFLAGS="-C target-cpu=native" cargo build --release```

# Benchmark
## On the *root* directory
### Printing out *root* directory
`ff` is faster by 13.121s
```
time ff "*" /

real    0m7.376s
user    0m0.427s
sys     0m1.654s
```

```
time fd . /

real    0m20.497s
user    0m3.654s
sys     0m10.308s
```

### Printing out *root* directory, redirecting output to ```/dev/null```
`ff` is faster by 1.804s
```
time ff "*" / > /dev/null

real    0m0.868s
user    0m0.801s
sys     0m1.459s
```

```
time fd . / > /dev/null

real    0m2.672s
user    0m1.972s
sys     0m2.736s
```

## On the *home* directory
### Printing out *home* directory
`fd` is faster by 0.294s
```
time ff "*" ~/

real    0m0.467s
user    0m0.034s
sys     0m0.057s
```

```
time fd . ~/

real    0m0.173s
user    0m0.026s
sys     0m0.053s
```

### Printing out *home* directory, redirecting output to ```/dev/null```
`fd` is faster by 0.001s (wow such gains!)
```
time ff "*" ~/ > /dev/null

real    0m0.024s
user    0m0.010s
sys     0m0.046s
```

```
time fd . ~/ > /dev/null

real    0m0.023s
user    0m0.015s
sys     0m0.027s
```