use std::env;
use serenity::
{
    async_trait,
    model::
    {
        channel::Message,
        gateway::Ready
    },
    prelude::*
};

struct Handler;

#[async_trait]
impl EventHandler for Handler
{
    async fn ready(&self, _: Context, ready: Ready)
    {
        println!("{} připraven", ready.user.name)
    }
}

#[tokio::main]
async fn main()
{
    dotenv::dotenv()
        .expect("Soubor .env nenalezen");
    let token = env::var("DISCORD_BOT")
        .expect("Očekávaný token Discord aplikace nenalezen");
    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Chyba při vytváření klienta");

    if let Err(why) = client.start().await
    {
        println!("Chyba: {:?}", why);
    }
}