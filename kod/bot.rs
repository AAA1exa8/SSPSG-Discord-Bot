mod prikaz;

use log::
{
    info,
    error
};
use std::env;
use poise::serenity_prelude as serenity;
use serenity::GatewayIntents;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn prikaz(context: Context<'_>) -> Result<(), Error>
{
    context.say("Příkaz").await?;
    Ok(())
}

#[tokio::main]
async fn main()
{
    // Inicializace záznamu log
    env_logger::init();

    // Načtení hodnot v souboru .env
    dotenv::dotenv()
        .expect("Soubor .env nenalezen");

    let token = env::var("DISCORD_BOT")
        .expect("Očekávaný token Discord aplikace nenalezen");
    let intents = GatewayIntents::non_privileged()
    | GatewayIntents::MESSAGE_CONTENT;
    let prikazy = vec!
    [
        prikaz()
    ];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions
            {
                commands: (prikazy),
                ..Default::default()
            })
        .token(token)
        .intents(intents)
        .setup(|context, _ready, framework|
        {
            Box::pin(async move
            {
                poise::builtins::register_globally(context, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}