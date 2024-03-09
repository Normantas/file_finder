# File Finder
### A "file finding" utility
To build from source and include native optimizations (specific to your CPU), do: ```RUSTFLAGS="-C target-cpu=native" cargo build --release```

## Benchmark: Comparison with ```time``` command
### When printing output to terminal
(Note: "cls" is an alias i set for "clear")
```
cls; time ./file_finder -i "*" ~/

# Snip! You probably don't need to see my home directory

real    0m0.574s
user    0m0.440s
sys     0m0.756s
```

```
cls; time find ~/

# Snip! You probably don't need to see my home directory

real    0m3.906s
user    0m1.108s
sys     0m1.421s
```

### When redirecting output to ```/dev/null```
```
time ./file_finder -i "*" ~/ > /dev/null

real    0m0.529s
user    0m0.571s
sys     0m0.754s
```

```
time find ~/ > /dev/null

real    0m0.655s
user    0m0.132s
sys     0m0.519s
```

# Current issues
When running with the -i flag, sometimes it doesnt find the desired file/directory even if it exists