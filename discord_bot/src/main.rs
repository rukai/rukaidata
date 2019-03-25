pub mod characters;

use std::env;

use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::prelude::*;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.author.name != "rukaidata" {
            let lower = msg.content.trim().to_lowercase();
            let tokens: Vec<_> = lower.split_whitespace().collect();

            if let Some(command) = tokens.get(0) {
                if *command == ".brawldata" || *command == ".pmdata" || *command == ".pm3.6data" || *command == ".p+data" || *command == ".lxpdata" || *command == ".lxp2.1data" {
                    let mod_path = match command.as_ref() {
                        ".brawldata" => "Brawl",
                        ".pmdata" => "PM3.6",
                        ".pm3.6data" => "PM3.6",
                        ".p+data" => "P+",
                        ".lxpdata" => "LXP2.1",
                        ".lxp2.1data" => "LXP2.1",
                        _ => unreachable!(),
                    };

                    // Rather than actually checking sequences of tokens, I just check the first word of a characters name,
                    // I can get away with this because there aren't really any collisions.
                    let mut character = None;
                    for token in &tokens {
                        character = match mod_path {
                            "Brawl"  => characters::brawl(token),
                            "PM3.6"  => characters::brawl(token).or_else(|| characters::pm(token)),
                            "P+"     => characters::brawl(token).or_else(|| characters::pm(token)),
                            "LXP2.1" => characters::lxp(token),
                            _ => unreachable!(),
                        };

                        if character.is_some() {
                            break;
                        }
                    }

                    // TODO: This should be made a vector to contain all the subactions related to the action. (multiple jabs, smash attack startup/attack)
                    // TODO: Manually handle character specific stuff such as jabs, glides, etc
                    let mut subaction = None;

                    // jabs
                    if tokens.contains(&"jab") { subaction = Some("Attack11") }

                    // dash attack
                    if tokens.contains(&"dash") && tokens.contains(&"attack") { subaction = Some("AttackDash") }
                    if tokens.contains(&"dashattack")                         { subaction = Some("AttackDash") }

                    // grabs
                    if tokens.contains(&"grab")                              { subaction = Some("Catch") }
                    if tokens.contains(&"dash") && tokens.contains(&"grab")  { subaction = Some("CatchDash") }
                    if tokens.contains(&"dashgrab")                          { subaction = Some("CatchDash") }
                    if tokens.contains(&"turn") && tokens.contains(&"grab")  { subaction = Some("CatchTurn") }
                    if tokens.contains(&"turngrab")                          { subaction = Some("CatchTurn") }

                    // tilts
                    if tokens.contains(&"up")      && tokens.contains(&"tilt") { subaction = Some("AttackHi3") }
                    if tokens.contains(&"uptilt")                              { subaction = Some("AttackHi3") }
                    if tokens.contains(&"utilt")                               { subaction = Some("AttackHi3") }
                    if tokens.contains(&"down")    && tokens.contains(&"tilt") { subaction = Some("AttackLw3") }
                    if tokens.contains(&"downtilt")                            { subaction = Some("AttackLw3") }
                    if tokens.contains(&"dtilt")                               { subaction = Some("AttackLw3") }
                    if tokens.contains(&"forward") && tokens.contains(&"tilt") { subaction = Some("AttackS3S") }
                    if tokens.contains(&"forwardtilt")                         { subaction = Some("AttackS3S") }
                    if tokens.contains(&"ftilt")                               { subaction = Some("AttackS3S") }
                    if tokens.contains(&"side")    && tokens.contains(&"tilt") { subaction = Some("AttackS3S") }
                    if tokens.contains(&"sidetilt")                            { subaction = Some("AttackS3S") }
                    if tokens.contains(&"stilt")                               { subaction = Some("AttackS3S") }

                    // crawl attack
                    if (tokens.contains(&"crawl") && tokens.contains(&"attack")) ||
                       (tokens.contains(&"crawl") && tokens.contains(&"tilt"))   ||
                        tokens.contains(&"ctilt") {
                        match character {
                            Some("Lucario") => subaction = Some("AttackSquat"),
                            Some("Squirtle") => subaction = Some("AttackSquat"),
                            Some("Snake") => subaction = Some("AttackLwShank"),
                            Some("Samus") => subaction = Some("SpecialSDash"),
                            _ => { }
                        }
                    }

                    // Smashes
                    if tokens.contains(&"up")      && tokens.contains(&"smash") { subaction = Some("AttackHi4") }
                    if tokens.contains(&"upsmash")                              { subaction = Some("AttackHi4") }
                    if tokens.contains(&"usmash")                               { subaction = Some("AttackHi4") }
                    if tokens.contains(&"down")    && tokens.contains(&"smash") { subaction = Some("AttackLw4") }
                    if tokens.contains(&"downsmash")                            { subaction = Some("AttackLw4") }
                    if tokens.contains(&"dsmash")                               { subaction = Some("AttackLw4") }
                    if tokens.contains(&"forward") && tokens.contains(&"smash") { subaction = Some("AttackS4S") }
                    if tokens.contains(&"forwardsmash")                         { subaction = Some("AttackS4S") }
                    if tokens.contains(&"fsmash")                               { subaction = Some("AttackS4S") }

                    // aerials
                    if tokens.contains(&"up")      && tokens.contains(&"air") { subaction = Some("AttackAirHi") }
                    if tokens.contains(&"upair")                              { subaction = Some("AttackAirHi") }
                    if tokens.contains(&"uair")                               { subaction = Some("AttackAirHi") }
                    if tokens.contains(&"down")    && tokens.contains(&"air") { subaction = Some("AttackAirLw") }
                    if tokens.contains(&"downair")                            { subaction = Some("AttackAirLw") }
                    if tokens.contains(&"dair")                               { subaction = Some("AttackAirLw") }
                    if tokens.contains(&"forward") && tokens.contains(&"air") { subaction = Some("AttackAirF") }
                    if tokens.contains(&"forwardair")                         { subaction = Some("AttackAirF") }
                    if tokens.contains(&"fair")                               { subaction = Some("AttackAirF") }
                    if tokens.contains(&"back")    && tokens.contains(&"air") { subaction = Some("AttackAirB") }
                    if tokens.contains(&"backair")                            { subaction = Some("AttackAirB") }
                    if tokens.contains(&"bair")                               { subaction = Some("AttackAirB") }
                    if tokens.contains(&"neutral") && tokens.contains(&"air") { subaction = Some("AttackAirN") }
                    if tokens.contains(&"neutralair")                         { subaction = Some("AttackAirN") }
                    if tokens.contains(&"nair")                               { subaction = Some("AttackAirN") }

                    // specials
                    if tokens.contains(&"up")      && tokens.contains(&"special") { subaction = Some("SpecialHi") }
                    if tokens.contains(&"up")      && tokens.contains(&"b")       { subaction = Some("SpecialHi") }
                    if tokens.contains(&"upspecial")                              { subaction = Some("SpecialHi") }
                    if tokens.contains(&"upb")                                    { subaction = Some("SpecialHi") }
                    if tokens.contains(&"down")    && tokens.contains(&"special") { subaction = Some("SpecialLw") }
                    if tokens.contains(&"down")    && tokens.contains(&"b")       { subaction = Some("SpecialLw") }
                    if tokens.contains(&"downspecial")                            { subaction = Some("SpecialLw") }
                    if tokens.contains(&"downb")                                  { subaction = Some("SpecialLw") }
                    if tokens.contains(&"neutral") && tokens.contains(&"special") { subaction = Some("SpecialN") }
                    if tokens.contains(&"neutral") && tokens.contains(&"b")       { subaction = Some("SpecialN") }
                    if tokens.contains(&"neutralspecial")                         { subaction = Some("SpecialN") }
                    if tokens.contains(&"neutralb")                               { subaction = Some("SpecialN") }
                    if tokens.contains(&"forward") && tokens.contains(&"special") { subaction = Some("SpecialS") }
                    if tokens.contains(&"forward") && tokens.contains(&"b")       { subaction = Some("SpecialS") }
                    if tokens.contains(&"forwardspecial")                         { subaction = Some("SpecialS") }
                    if tokens.contains(&"forwardb")                               { subaction = Some("SpecialS") }
                    if tokens.contains(&"side")    && tokens.contains(&"special") { subaction = Some("SpecialS") }
                    if tokens.contains(&"side")    && tokens.contains(&"b")       { subaction = Some("SpecialS") }
                    if tokens.contains(&"sidespecial")                            { subaction = Some("SpecialS") }
                    if tokens.contains(&"sideb")                                  { subaction = Some("SpecialS") }

                    // specials air
                    if tokens.contains(&"air") && tokens.contains(&"up")      && tokens.contains(&"special") { subaction = Some("SpecialAirHi") }
                    if tokens.contains(&"air") && tokens.contains(&"up")      && tokens.contains(&"b")       { subaction = Some("SpecialAirHi") }
                    if tokens.contains(&"air") && tokens.contains(&"upspecial")                              { subaction = Some("SpecialAirHi") }
                    if tokens.contains(&"air") && tokens.contains(&"upb")                                    { subaction = Some("SpecialAirHi") }
                    if tokens.contains(&"air") && tokens.contains(&"down")    && tokens.contains(&"special") { subaction = Some("SpecialAirLw") }
                    if tokens.contains(&"air") && tokens.contains(&"down")    && tokens.contains(&"b")       { subaction = Some("SpecialAirLw") }
                    if tokens.contains(&"air") && tokens.contains(&"downspecial")                            { subaction = Some("SpecialAirLw") }
                    if tokens.contains(&"air") && tokens.contains(&"downb")                                  { subaction = Some("SpecialAirLw") }
                    if tokens.contains(&"air") && tokens.contains(&"neutral") && tokens.contains(&"special") { subaction = Some("SpecialAirN") }
                    if tokens.contains(&"air") && tokens.contains(&"neutral") && tokens.contains(&"b")       { subaction = Some("SpecialAirN") }
                    if tokens.contains(&"air") && tokens.contains(&"neutralspecial")                         { subaction = Some("SpecialAirN") }
                    if tokens.contains(&"air") && tokens.contains(&"neutralb")                               { subaction = Some("SpecialAirN") }
                    if tokens.contains(&"air") && tokens.contains(&"forward") && tokens.contains(&"special") { subaction = Some("SpecialAirS") }
                    if tokens.contains(&"air") && tokens.contains(&"forward") && tokens.contains(&"b")       { subaction = Some("SpecialAirS") }
                    if tokens.contains(&"air") && tokens.contains(&"forwardspecial")                         { subaction = Some("SpecialAirS") }
                    if tokens.contains(&"air") && tokens.contains(&"forwardb")                               { subaction = Some("SpecialAirS") }
                    if tokens.contains(&"air") && tokens.contains(&"side")    && tokens.contains(&"special") { subaction = Some("SpecialAirS") }
                    if tokens.contains(&"air") && tokens.contains(&"side")    && tokens.contains(&"b")       { subaction = Some("SpecialAirS") }
                    if tokens.contains(&"air") && tokens.contains(&"sidespecial")                            { subaction = Some("SpecialAirS") }
                    if tokens.contains(&"air") && tokens.contains(&"sideb")                                  { subaction = Some("SpecialAirS") }

                    let message = match (character, subaction) {
                        (Some(character), Some(subaction)) => format!("https://rukaidata.com/{}/{}/subactions/{}.html", mod_path, character, subaction),
                        (Some(character), None)            => format!("https://rukaidata.com/{}/{}", mod_path, character),
                        (None,            Some(_))         => format!("Need to specify a character."),
                        (None, None)                       => format!("https://rukaidata.com/{}", mod_path),
                    };

                    send(&ctx, &msg.channel_id, &message);
                }

                if *command == ".rattening" {
                    send(&ctx, &msg.channel_id, "🐀🐀🐀 https://www.youtube.com/watch?v=qXEtmSi36AI");
                }
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn send(ctx: &Context, channel_id: &ChannelId, text: &str) {
    if let Err(why) = channel_id.say(&ctx.http, text) {
        println!("Error sending message: {:?}", why);
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
