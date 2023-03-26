mod prikaz;

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
        prelude::*
    },
    prelude::*
};

struct Bot;

#[async_trait]
impl EventHandler for Bot
{
    async fn interaction_create(&self, context: Context, interaction: Interaction)
    {
        if let Interaction::ApplicationCommand(command) = interaction
        {
            info!("Interakce {:#?} přijata", command);

            let content = match command.data.name.as_str() {
                "Prikaz" => prikaz::prikaz::run(),
                _ => "Neimplementováno".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&context.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                error!("Cannot respond to slash command: {}", why);
            }

        }
    }

    async fn ready(&self, context: Context, ready: Ready)
    {
        env_logger::init();
        info!("Discord Bot {} připraven", ready.user.name);

        let commands = Command::create_global_application_command(context.http, |command| 
        {
            prikaz::prikaz::register(command)
        }).await;

        info!("Příkazy {:#?} registrovány", commands)
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