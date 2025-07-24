# casex (bash prompt)
CigarettesAfterSex Online Prompt minimal for Termux

> Motivating: To disguise the @ symbol prefix bug that was hard to get rid of in the default termux prompt (bash), I made a Casex prompt to solve the bad prompt problem in termux to be equivalent to PS1 zsh or fish.

# Prerequesite
* rustc & cargo
* python3 (package)
* hr (package)
* bash-preexec (plugin)
  
# Build from source
```sh
rustc -o casex main.rs && mv casex ../usr/bin
```

# Hook or sourcing
### my preference configurations at $PREFIX/usr/etc/profile.d/~~casex.sh

```sh
## Configurations
#Comment this line if you want a dynamic prompt. (Komentari baris ini jika Anda ingin tampilan prompt dinamis)
trap '' WINCH
# This make prompt statically (Ini akan membuat prompt menjadi statis)
# Hook prompt
eval "$(casex init bash)"
```
# Last Testing on Termux Version 0.119.0-beta.3 (1022) 
# License
