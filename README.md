# To build for Linux on a Mac

```
docker pull clux/muslrust
docker run -v $PWD:/volume -t clux/muslrust cargo build --release
```

And push the binary out:

```
scp target/x86_64-unknown-linux-musl/release/onthehour server.someplace:
```

# How to create Twitter credentials

Go to https://apps.twitter.com/ and "Create a new App".
Once that is done, in the "Keys and Access Tokens" tab, 
make a note of:

- Consumer Key (API Key)
- Consumer Secret (API Secret)

Scroll down and press "Create my access token", and then note:

- Access Token
- Access Token Secret

When you run `onthehour` do so with the following environment variables...

```
CONSUMER_KEY=??? CONSUMER_SECRET=??? ACCESS_TOKEN=??? ACCESS_SECRET=??? onthehour
```
