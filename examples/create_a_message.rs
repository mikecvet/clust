//! This example demonstrates how to use the `create_a_message` API.
//!
//! ```shell
//! $ cargo run --example create_a_message -- -p <prompt> -m <message>
//! ```
//!
//! e.g.
//! ```shell
//! $ cargo run --example create_a_message -- -p "You are a excellent AI assistant." -m "Where is the capital of Japan?"
//! ```

use clust::messages::ClaudeModel;
use clust::messages::Content;
use clust::messages::ContentBlock;
use clust::messages::MaxTokens;
use clust::messages::Message;
use clust::messages::MessagesRequestBody;
use clust::messages::MessagesResponseBody;
use clust::messages::SystemPrompt;
use clust::messages::TextContentBlock;
use clust::Client;

use clap::Parser;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    prompt: String,
    #[arg(short, long)]
    message: String,
}

/// Demonstrates how to extract the response text from a MessagesResponseBody, and prints to stdout
fn print_response_text(response: MessagesResponseBody) {
  match response.content {
    Content::MultipleBlock(response_vector) => {
      if !response_vector.is_empty() {
        for block in response_vector.iter() {
          match block {
            ContentBlock::Text(TextContentBlock { _type, text }) => println!("Multi-block response text: {text}"),
            _ => ()
          };
        }
      }
    },
    Content::SingleText(text) => println!("Single text response: {text}")
  };
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 0. Parse the command-line arguments.
    let arguments = Arguments::parse();

    // 1. Create a new API client with the API key loaded from the environment variable: `ANTHROPIC_API_KEY`.
    let client = Client::from_env()?;

    // 2. Create a request body.
    let model = ClaudeModel::Claude3Haiku20240307;
    let messages = vec![Message::user(
        arguments.message,
    )];
    let max_tokens = MaxTokens::new(1024, model)?;
    let system_prompt = SystemPrompt::new(arguments.prompt);
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

    println!("Entire result:\n{}", response);

    print_response_text(response);

    Ok(())
}
