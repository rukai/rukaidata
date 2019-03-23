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
                if *command == ".pmdata" || *command == ".pm3.6data" || *command == ".lxpdata" || *command == ".lxp2.0data" {
                    let mod_path = match command.as_ref() {
                        ".pmdata" => "PM3.6",
                        ".pm3.6data" => "PM3.6",
                        ".lxpdata" => "LXP2.0",
                        ".lxp2.0data" => "LXP2.0",
                        _ => unreachable!(),
                    };

                    // Rather than actually checking sequences of tokens, I just check the first word of a characters name,
                    // I can get away with this because there aren't really any collisions.
                    let mut character = None;
                    for token in &tokens {
                        character = match token.as_ref() {
                            "bowser"          => Some("Bowser"),
                            "captain"         => Some("Captain%20Falcon"),
                            "captainfalcon"   => Some("Captain%20Falcon"),
                            "charizard"       => Some("Charizard"),
                            "diddy"           => Some("Diddy%20Kong"),
                            "diddykong"       => Some("Diddy%20Kong"),
                            "donkey"          => Some("Donkey%20Kong"),
                            "donkeykong"      => Some("Donkey%20Kong"),
                            "dk"              => Some("Donkey%20Kong"),
                            "falco"           => Some("Falco"),
                            "fox"             => Some("Fox"),
                            "game"            => Some("Game%20&%20Watch"),
                            "game&watch"      => Some("Game%20&%20Watch"),
                            "gameandwatch"    => Some("Game%20&%20Watch"),
                            "g&w"             => Some("Game%20&%20Watch"),
                            "gaw"             => Some("Game%20&%20Watch"),
                            "ganondorf"       => Some("Ganondorf"),
                            "ganon"           => Some("Ganondorf"),
                            "dorf"            => Some("Ganondorf"),
                            "giga bowser"     => Some("Giga%20Bowser"),
                            "gigabowser"      => Some("Giga%20Bowser"),
                            "gb"              => Some("Giga%20Bowser"),
                            "iceclimbers"     => Some("Ice%20Climbers"),
                            "iceclimber"      => Some("Ice%20Climbers"),
                            "ice"             => Some("Ice%20Climbers"),
                            "ic"              => Some("Ice%20Climbers"),
                            "ics"             => Some("Ice%20Climbers"),
                            "ike"             => Some("Ike"),
                            "ivysaur"         => Some("Ivysaur"),
                            "ivy"             => Some("Ivysaur"),
                            "jigglypuff"      => Some("Jigglypuff"),
                            "jiggly"          => Some("Jigglypuff"),
                            "jiggs"           => Some("Jigglypuff"),
                            "kingdedede"      => Some("King%20Dedede"),
                            "king"            => Some("King%20Dedede"),
                            "dedede"          => Some("King%20Dedede"),
                            "d3"              => Some("King%20Dedede"),
                            "ddd"             => Some("King%20Dedede"),
                            "kd"              => Some("King%20Dedede"),
                            "kirby"           => Some("Kirby"),
                            "link"            => Some("Link"),
                            "lucario"         => Some("Lucario"),
                            "lucas"           => Some("Lucas"),
                            "luigi"           => Some("Luigi"),
                            "mario"           => Some("Mario"),
                            "marth"           => Some("Marth"),
                            "meta"            => Some("Meta%20Knight"),
                            "metaknight"      => Some("Meta%20Knight"),
                            "mk"              => Some("Meta%20Knight"),
                            "mewtwo"          => Some("Mewtwo"),
                            "m2"              => Some("Mewtwo"),
                            "ness"            => Some("Ness"),
                            "olimar"          => Some("Olimar"),
                            "peach"           => Some("Peach"),
                            "pikachu"         => Some("Pikachu"),
                            "pit"             => Some("Pit"),
                            "rob"             => Some("R.O.B"),
                            "r.o.b"           => Some("R.O.B"),
                            "roy"             => Some("Roy"),
                            "samus"           => Some("Samus"),
                            "sheik"           => Some("Sheik"),
                            "solid"           => Some("Snake"),
                            "solidsnake"      => Some("Snake"),
                            "snake"           => Some("Snake"),
                            "sonic"           => Some("Sonic"),
                            "squirtle"        => Some("Squirtle"),
                            "toon"            => Some("Toon%20Link"),
                            "toonlink"        => Some("Toon%20Link"),
                            "tl"              => Some("Toon%20Link"),
                            "wario"           => Some("Wario"),
                            "wario-man"       => Some("Wario-Man"),
                            "warioman"        => Some("Wario-Man"),
                            "wolf"            => Some("Wolf"),
                            "yoshi"           => Some("Yoshi"),
                            "zelda"           => Some("Zelda"),
                            "zero"            => Some("Zero Suit Samus"),
                            "zerosuitsamus"   => Some("Zero Suit Samus"),
                            "zss"             => Some("Zero Suit Samus"),
                            _                 => None,
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
