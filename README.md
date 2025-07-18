# casex
CigarettesAfterSex Online Prompt minimal for Termux

# Build
```sh
rustc -o casex main.rs && mv casex ../usr/bin
```

# Hook or sourcing
### my preference: cat ../usr/etc/profile.d/~~casex.sh

```sh
eval "$(casex init bash)"
```
