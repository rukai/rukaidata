pub mod characters;
pub mod subactions;

use std::env;

use chrono::Utc;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::model::interactions::application_command::{
    ApplicationCommand, ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionType,
};
use serenity::model::interactions::{Interaction, InteractionResponseType};
use serenity::prelude::*;

fn tokenize(msg: &str) -> Vec<String> {
    let lower = msg.trim().to_lowercase();
    lower.split_whitespace().map(|x| x.to_string()).collect()
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "rattening" => "🐀🐀🐀 https://www.youtube.com/watch?v=qXEtmSi36AI".to_string(),
                command_name => {
                    let fighter_option = match command
                        .data
                        .options
                        .get(0)
                        .unwrap()
                        .resolved
                        .as_ref()
                        .unwrap()
                    {
                        ApplicationCommandInteractionDataOptionValue::String(value) => value,
                        data => {
                            println!("Unexpected fighter arg {:?}", data);
                            return;
                        }
                    };
                    let subaction_option = match command
                        .data
                        .options
                        .get(1)
                        .unwrap()
                        .resolved
                        .as_ref()
                        .unwrap()
                    {
                        ApplicationCommandInteractionDataOptionValue::String(value) => value,
                        data => {
                            println!("Unexpected subaction arg {:?}", data);
                            return;
                        }
                    };

                    let mod_path = match command_name {
                        "data_brawl" => "Brawl",
                        "data_pm" => "PM3.6",
                        "data_pp" => "P+",
                        "data_lxp" => "LXP2.1",
                        _ => unreachable!(),
                    };

                    let fighter_tokens = tokenize(fighter_option);
                    let fighter_tokens: Vec<_> =
                        fighter_tokens.iter().map(|x| x.as_str()).collect();
                    let mut character = None;
                    for token in &fighter_tokens {
                        character = characters::character(mod_path, token);

                        if character.is_some() {
                            break;
                        }
                    }

                    let subaction_tokens = tokenize(subaction_option);
                    let subaction_tokens: Vec<_> =
                        subaction_tokens.iter().map(|x| x.as_str()).collect();
                    let subactions = subactions::subactions(&subaction_tokens, character);

                    println!("slash command {}", Utc::now().format("%F %T"));

                    match (character, subactions.is_empty()) {
                        (Some(character), false) => {
                            let mut message = String::new();
                            for subaction in &subactions {
                                if !message.is_empty() {
                                    message.push('\n');
                                }
                                message.push_str(&format!(
                                    "https://rukaidata.com/{}/{}/subactions/{}.html",
                                    mod_path, character, subaction
                                ));
                            }
                            message
                        }
                        (Some(character), true) => {
                            format!("https://rukaidata.com/{}/{}", mod_path, character)
                        }
                        (None, false) => "Need to specify a character.".to_string(),
                        (None, true) => format!("https://rukaidata.com/{}", mod_path),
                    }
                }
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
    #[rustfmt::skip]
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.name != "rukaidata" {
            let lower = msg.content.trim().to_lowercase();
            let tokens: Vec<_> = lower.split_whitespace().collect();

            if let Some(command) = tokens.get(0) {
                if *command == ".brawldata" || *command == ".pm3.02data" || *command == ".pm3.6data" || *command == ".p+data" || *command == ".xpdata" || *command == ".lxpdata" || *command == ".lxp2.1data"
                || *command == "!brawldata" || *command == "!pm3.02data" || *command == "!pm3.6data" || *command == "!p+data" || *command == "!xpdata" || *command == "!lxpdata" || *command == "!lxp2.1data" || *command == "!pmdata"
                || *command == "!secretdata" || *command == ".secretdata" {
                    let mod_path = match command[1..].as_ref() {
                        "brawldata"  => "Brawl",
                        "pmdata"     => "PM3.6",
                        "pm3.6data"  => "PM3.6",
                        "pm3.02data" => "PM3.02",
                        "p+data"     => "P+",
                        "xpdata"     => "LXP2.1",
                        "lxpdata"    => "LXP2.1",
                        "lxp2.1data" => "LXP2.1",
                        "secretdata" => "Secret",
                        _ => unreachable!(),
                    };

                    let mut character = None;
                    for token in &tokens {
                        character = characters::character(mod_path, token);

                        if character.is_some() {
                            break;
                        }
                    }

                    let subactions = subactions::subactions(&tokens, character);

                    let message = match (character, subactions.is_empty()) {
                        (Some(character), false) => {
                            let mut message = String::new();
                            for subaction in &subactions {
                                if !message.is_empty() {
                                    message.push('\n');
                                }
                                message.push_str(&format!("https://rukaidata.com/{}/{}/subactions/{}.html", mod_path, character, subaction));
                            }
                            message
                        }
                        (Some(character), true ) => format!("https://rukaidata.com/{}/{}", mod_path, character),
                        (None,            false) => "Need to specify a character.".to_string(),
                        (None,            true ) => format!("https://rukaidata.com/{}", mod_path),
                    };

                    send(&ctx, &msg.channel_id, &message).await;

                    println!("{}", Utc::now().format("%F %T"));
                }

                if *command == ".rattening" || *command == "!rattening" {
                    send(&ctx, &msg.channel_id, "🐀🐀🐀 https://www.youtube.com/watch?v=qXEtmSi36AI").await;
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let data_commands = [
            DataCommand {
                name: "data_brawl",
                description: "Display Brawl frame data",
            },
            DataCommand {
                name: "data_pm",
                description: "Display Project M 3.6 frame data",
            },
            DataCommand {
                name: "data_pp",
                description: "Display Project+ (latest release) frame data",
            },
            DataCommand {
                name: "data_lxp",
                description: "Display Legacy XP 2.1 frame data",
            },
        ];
        for data_command in data_commands {
            let command_result =
                ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                    command
                        .name(data_command.name)
                        .description(data_command.description)
                        .create_option(|option| {
                            option
                                .name("fighter")
                                .description("The name of the fighter")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                        })
                        .create_option(|option| {
                            option
                                .name("action")
                                .description("The name of the action")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                        })
                })
                .await;
            if let Err(err) = command_result {
                println!(
                    "Failed to create global slash command for {}: {}",
                    data_command.name, err
                );
            }
        }

        let command_result =
            ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                command
                    .name("rattening")
                    .description("Commence the rattening!")
            })
            .await;
        if let Err(err) = command_result {
            println!(
                "Failed to create global slash command for rattening: {}",
                err
            );
        }
        println!("{} is connected!", ready.user.name);
    }
}

struct DataCommand {
    name: &'static str,
    description: &'static str,
}

async fn send(ctx: &Context, channel_id: &ChannelId, text: &str) {
    if let Err(why) = channel_id.say(&ctx.http, text).await {
        println!("Error sending message: {:?}", why);
    }
}

#[tokio::main]
async fn main() {
    let discord_token =
        env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment");
    let application_id = env::var("APPLICATION_ID")
        .expect("Expected APPLICATION_ID in the environment")
        .parse()
        .expect("APPLICATION_ID must be numeric");

    let mut client = Client::builder(&discord_token)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
