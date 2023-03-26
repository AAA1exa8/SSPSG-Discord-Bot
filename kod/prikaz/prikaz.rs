use serenity::builder::CreateApplicationCommand;

pub fn run() -> String
{
    "Příkaz".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
{
    command
        .name("prikaz").description("Příkaz")
}