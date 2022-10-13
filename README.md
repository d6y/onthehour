# A bot that Tweets every hour

## Running

When you run `onthehour` do so with the following environment variables:

```
cargo build --release
CONSUMER_KEY=??? CONSUMER_SECRET=??? ACCESS_TOKEN=??? ACCESS_SECRET=??? onthehour
```

Or: use the `Dockerfile` to build a docker image.

## How to create Twitter credentials

Go to https://apps.twitter.com/ and "Create a new App".
Once that is done, in the "Keys and Access Tokens" tab, 
make a note of:

- Consumer Key (API Key)
- Consumer Secret (API Secret)

Scroll down and press "Create my access token", and then note:

- Access Token
- Access Token Secret

