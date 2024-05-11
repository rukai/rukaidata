// This lint is stupid.
// I need an import and an unwrap to use `write!`, make the API more ergnomic before forcing it on me.
#![allow(clippy::format_push_string)]

pub mod characters;
pub mod subactions;

use poise::serenity_prelude as serenity;

fn tokenize(msg: &str) -> Vec<String> {
    let lower = msg.trim().to_lowercase();
    lower.split_whitespace().map(|x| x.to_string()).collect()
}

fn timestamp() -> String {
    time::OffsetDateTime::now_utc().to_string()
}

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Commence the rattening!
#[poise::command(slash_command, prefix_command)]
async fn rattening(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("🐀🐀🐀 https://www.youtube.com/watch?v=qXEtmSi36AI".to_string())
        .await?;
    Ok(())
}

/// Display Brawl frame data
#[poise::command(slash_command)]
async fn databrawl(
    ctx: Context<'_>,
    #[description = "The name of the fighter"] fighter: String,
    #[description = "The name of the action"] action: String,
) -> Result<(), Error> {
    data("Brawl", ctx, fighter, action).await
}

/// Display Project+ (latest release) frame data
#[poise::command(slash_command)]
async fn datapplus(
    ctx: Context<'_>,
    #[description = "The name of the fighter"] fighter: String,
    #[description = "The name of the action"] action: String,
) -> Result<(), Error> {
    data("P+", ctx, fighter, action).await
}

/// Display Legacy XP 2.1 frame data
#[poise::command(slash_command)]
async fn datalxp(
    ctx: Context<'_>,
    #[description = "The name of the fighter"] fighter: String,
    #[description = "The name of the action"] action: String,
) -> Result<(), Error> {
    data("LXP2.1", ctx, fighter, action).await
}

/// Display Project M 3.6 frame data
#[poise::command(slash_command)]
async fn datapm(
    ctx: Context<'_>,
    #[description = "The name of the fighter"] fighter: String,
    #[description = "The name of the action"] action: String,
) -> Result<(), Error> {
    data("PM3.6", ctx, fighter, action).await
}

async fn data(
    mod_path: &str,
    ctx: Context<'_>,
    fighter: String,
    subaction: String,
) -> Result<(), Error> {
    let fighter_tokens = tokenize(&fighter);
    let fighter_tokens: Vec<_> = fighter_tokens.iter().map(|x| x.as_str()).collect();
    let mut character = None;
    for token in &fighter_tokens {
        character = characters::character(mod_path, token);

        if character.is_some() {
            break;
        }
    }

    let character = match character {
        Some(character) => character,
        None => {
            return Err(
                format!("fighter `{}` does not exist in mod `{}`", fighter, mod_path).into(),
            )
        }
    };

    let subaction_tokens = tokenize(&subaction);
    let subaction_tokens: Vec<_> = subaction_tokens.iter().map(|x| x.as_str()).collect();
    let subactions = subactions::subactions(&subaction_tokens, character);
    if subactions.is_empty() {
        return Err(format!(
            "action `{}` does not exist on fighter `{}` in mod `{}`",
            subaction, fighter, mod_path
        )
        .into());
    }

    println!("slash command {}", timestamp());

    let output = subactions
        .iter()
        .map(|subaction| {
            format!(
                "https://rukaidata.com/{}/{}/subactions/{}.html",
                mod_path, character, subaction
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    ctx.say(output).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();
    let commands = vec![rattening(), databrawl(), datapplus(), datapm(), datalxp()];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
