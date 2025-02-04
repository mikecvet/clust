# clust

An unofficial Rust client
for [the Anthropic/Claude API](https://docs.anthropic.com/claude/reference/getting-started-with-the-api).

## Installation

Run the following Cargo command in your project directory:

```shell
cargo add clust
```

or add the following line to your Cargo.toml:

```toml
[dependencies]
clust = "0.5.0"
```

## Supported APIs

- Messages
    - [x] [Create a Message](https://docs.anthropic.com/claude/reference/messages_post)
    - [x] [Streaming Messages](https://docs.anthropic.com/claude/reference/messages-streaming)

## Usage

### Create a message

An example of creating a message with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`

```env
ANTHROPIC_API_KEY={your-api-key}
```

is as follows:

```rust
use clust::messages::ClaudeModel;
use clust::messages::MaxTokens;
use clust::messages::Message;
use clust::messages::MessagesRequestBody;
use clust::messages::SystemPrompt;
use clust::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Create a new API client with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`.
    let client = Client::from_env()?;

    // 2. Create a request body.
    let model = ClaudeModel::Claude3Sonnet20240229;
    let messages = vec![Message::user(
        "Where is the capital of France?",
    )];
    let max_tokens = MaxTokens::new(1024, model)?;
    let system_prompt = SystemPrompt::new("You are an excellent AI assistant.");
    let request_body = MessagesRequestBody {
        model,
        messages,
        max_tokens,
        system: Some(system_prompt),
        ..Default::default()
    };

    // 3. Call the API.
    let response = client
        .create_a_message(request_body)
        .await?;

    println!("Result:\n{}", response);

    Ok(())
}
```

### Streaming messages

An example of creating a message stream with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`

```env
ANTHROPIC_API_KEY={your-api-key}
```

with [futures_util](https://crates.io/crates/futures-util) is as follows:

```rust
use clust::messages::ClaudeModel;
use clust::messages::MaxTokens;
use clust::messages::Message;
use clust::messages::MessagesRequestBody;
use clust::messages::SystemPrompt;
use clust::messages::StreamOption;
use clust::messages::StreamChunk;
use clust::Client;

use futures_util::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Create a new API client with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`.
    let client = Client::from_env()?;

    // 2. Create a request body with `stream` option.
    let model = ClaudeModel::Claude3Sonnet20240229;
    let messages = vec![Message::user(
        "Where is the capital of France?",
    )];
    let max_tokens = MaxTokens::new(1024, model)?;
    let system_prompt = SystemPrompt::new("You are an excellent AI assistant.");
    let request_body = MessagesRequestBody {
        model,
        messages,
        max_tokens,
        system: Some(system_prompt),
        stream: Some(StreamOption::ReturnStream),
        ..Default::default()
    };

    // 3. Call the streaming API.
    let mut stream = client
        .create_a_message_stream(request_body)
        .await?;

    let mut buffer = String::new();

    // 4. Poll the stream.
    // NOTE: The `futures_util::StreamExt` run on the single thread.
    while let Some(chunk) = stream.next().await {
        match chunk {
            | Ok(chunk) => {
                println!("Chunk:\n{}", chunk);
                match chunk {
                    | StreamChunk::ContentBlockDelta(content_block_delta) => {
                        // Buffer message delta.
                        buffer.push_str(&content_block_delta.delta.text);
                    }
                    | _ => {}
                }
            }
            | Err(error) => {
                eprintln!("Chunk error:\n{:?}", error);
            }
        }
    }

    println!("Result:\n{}", buffer);

    Ok(())
}
```

### Streaming messages with `tokio` backend

An example of creating a message stream with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`

```env
ANTHROPIC_API_KEY={your-api-key}
```

with [tokio-stream](https://docs.rs/tokio-stream/latest/tokio_stream/) is as follows:

```rust
use clust::messages::ClaudeModel;
use clust::messages::MaxTokens;
use clust::messages::Message;
use clust::messages::MessagesRequestBody;
use clust::messages::SystemPrompt;
use clust::messages::StreamOption;
use clust::messages::StreamChunk;
use clust::Client;

use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Create a new API client with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`.
    let client = Client::from_env()?;

    // 2. Create a request body with `stream` option.
    let model = ClaudeModel::Claude3Sonnet20240229;
    let messages = vec![Message::user(
        "Where is the capital of France?",
    )];
    let max_tokens = MaxTokens::new(1024, model)?;
    let system_prompt = SystemPrompt::new("You are an excellent AI assistant.");
    let request_body = MessagesRequestBody {
        model,
        messages,
        max_tokens,
        system: Some(system_prompt),
        stream: Some(StreamOption::ReturnStream),
        ..Default::default()
    };

    // 3. Call the streaming API.
    let mut stream = client
        .create_a_message_stream(request_body)
        .await?;

    let mut buffer = String::new();

    // 4. Poll the stream.
    // NOTE: The `tokio_stream::StreamExt` run on the `tokio` runtime.
    while let Some(chunk) = stream.next().await {
        match chunk {
            | Ok(chunk) => {
                println!("Chunk:\n{}", chunk);
                match chunk {
                    | StreamChunk::ContentBlockDelta(content_block_delta) => {
                        // Buffer message delta.
                        buffer.push_str(&content_block_delta.delta.text);
                    }
                    | _ => {}
                }
            }
            | Err(error) => {
                eprintln!("Chunk error:\n{:?}", error);
            }
        }
    }

    println!("Result:\n{}", buffer);

    Ok(())
}
```

## Other examples

See the [examples](./examples) directory for more examples.

## Changelog

See [CHANGELOG](./CHANGELOG.md).

## License

Licensed under either of the [Apache License, Version 2.0](./LICENSE-APACHE) or the [MIT](./LICENSE-MIT) license at your
option.
