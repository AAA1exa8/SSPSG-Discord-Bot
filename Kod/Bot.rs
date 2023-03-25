use log::{info, error};
use std::env;
use serenity::
{
    async_trait,
    model::
    {
        application::command::Command,
        application::interaction::
        {
            Interaction,
            InteractionResponseType
        },
        gateway::Ready,
        prelude::{Guild, GuildId, command}
    },
    prelude::*
};

struct Bot;

#[async_trait]
impl EventHandler for Bot
{
    async fn ready(&self, context: Context, ready: Ready)
    {
        env_logger::init();
        info!("{} připraven", ready.user.name);
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
        .event_handler(Bot)
        .await
        .expect("Chyba při vytváření klienta");

    if let Err(why) = client.start().await
    {
        error!("Chyba: {:?}", why);
    }
}