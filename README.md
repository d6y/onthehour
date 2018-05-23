# To build for Linux on a Mac

```
docker pull clux/muslrust
docker run -v $PWD:/volume -t clux/muslrust cargo build --release
```

And push the binary out:

```
scp target/x86_64-unknown-linux-musl/release/onthehour server.someplace:

