#[tokio::main]
async fn main() {
    use twilight_gateway::StreamExt;

    dotenvy::dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").unwrap();
    let http = twilight_http::Client::new(token.clone());
    // なんか両方いるっぽい
    let intents = twilight_gateway::Intents::GUILD_MESSAGES
        | twilight_gateway::Intents::MESSAGE_CONTENT;
    let mut shaed = twilight_gateway::Shard::new(
        twilight_gateway::ShardId::ONE,
        token.clone(),
        intents,
    );

    while let Some(item) = shaed
        .next_event(twilight_gateway::EventTypeFlags::all())
        .await
    {
        let Ok(event) = item else {
            eprintln!("error receiving event: {:?}", item);
            continue;
        };

        match event {
            twilight_gateway::Event::MessageCreate(msg) => {
                println!("message received with content: {}", msg.content);

                if msg.content == "!ping" {
                    let _ = http.create_message(msg.channel_id).content("Pong!").await;
                }
            }
            twilight_gateway::Event::MessageDelete(msg) => {
                println!("message with id`{}` deleted", msg.id);
            }
            _ => {}
        }
    }
}
