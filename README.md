# rust-gcp-pubsub (Finhaven Forked Version)

A crate that acts as a HTTP client to publish and read messages from Google Cloud Platform's PubSub. Forked and updated
from the original [gcp-pubsub](https://lib.rs/crates/gcp-pubsub) project.

## Usage

#### Create a client

Authentication is provided by [rust-goauth](https://github.com/durch/rust-goauth), which expects a path to the file containing your Google Cloud service account JSON key.

```rust
let google_credentials = std::env::var("GOOGLE_PUBSUB_CREDENTIALS").unwrap();
let mut client = gcp_pubsub::Client::new(credentials);
```

#### Create a topic

```rust
let topic = client.create_topic("my-topic").await;
```

#### Publish a message

```rust
#[derive(Serialize, Default)]
struct Foo {
  pub a: String,
}

let result: Result<(), Error> = topic.publish(Foo::default()).await;
```
