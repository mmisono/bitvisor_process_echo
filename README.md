# bitvisor_process_echo
## requirements
- rust
- xargo

## rust setup
```
curl https://sh.rustup.rs -sSf | sh
rustup override set nightly
cargo install xargo
```

## compile

```
hg clone https://bitbucket.org/bitvisor/bitvisor
cd bitvisor/process
git clone https://github.com/mmisono/bitvisor_process_echo
cd ../
patch -p1 < ./process/bitvisor_process_echo/patch.diff
rustup override set nightly
make
```

## check
```
./dbgsh
> echo
echo> foo
foo
echo> exit
>
```

## dependencies
- [bitvisor_process_lib](https://github.com/mmisono/bitvisor_process_lib)
