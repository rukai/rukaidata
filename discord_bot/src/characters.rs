pub fn character(mod_path: &str, fighter_option: &str) -> Option<&'static str> {
    // Rather than actually checking sequences of tokens, I just check the first word of a characters name,
    // I can get away with this because there aren't really any collisions.
    match mod_path {
        "Brawl" => brawl(fighter_option),
        "PM3.02" => brawl(fighter_option).or_else(|| pm(fighter_option)),
        "PM3.6" => brawl(fighter_option).or_else(|| pm(fighter_option)),
        "P+" => brawl(fighter_option)
            .or_else(|| pm(fighter_option))
            .or_else(|| pplus(fighter_option)),
        "LXP2.1" => lxp(fighter_option),
        "Secret" => secret(fighter_option),
        _ => unreachable!(),
    }
}

#[rustfmt::skip]
pub fn brawl(token: &str) -> Option<&'static str> {
    match token {
        "bowser"          => Some("Bowser"),
        "captain"         => Some("Captain%20Falcon"),
        "falcon"          => Some("Captain%20Falcon"),
        "captainfalcon"   => Some("Captain%20Falcon"),
        "cf"              => Some("Captain%20Falcon"),
        "charizard"       => Some("Charizard"),
        "zard"            => Some("Charizard"),
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
        "gamewatch"       => Some("Game%20&%20Watch"),
        "g&w"             => Some("Game%20&%20Watch"),
        "gaw"             => Some("Game%20&%20Watch"),
        "gw"              => Some("Game%20&%20Watch"),
        "gnw"             => Some("Game%20&%20Watch"),
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
        "ices"            => Some("Ice%20Climbers"),
        "icies"           => Some("Ice%20Climbers"),
        "ike"             => Some("Ike"),
        "ivysaur"         => Some("Ivysaur"),
        "ivy"             => Some("Ivysaur"),
        "jigglypuff"      => Some("Jigglypuff"),
        "jiggly"          => Some("Jigglypuff"),
        "jiggs"           => Some("Jigglypuff"),
        "puff"            => Some("Jigglypuff"),
        "kingdedede"      => Some("King%20Dedede"),
        "king"            => Some("King%20Dedede"),
        "dedede"          => Some("King%20Dedede"),
        "d3"              => Some("King%20Dedede"),
        "ddd"             => Some("King%20Dedede"),
        "kd"              => Some("King%20Dedede"),
        "kirby"           => Some("Kirby"),
        "kirb"            => Some("Kirby"),
        "link"            => Some("Link"),
        "lucario"         => Some("Lucario"),
        "lucas"           => Some("Lucas"),
        "luigi"           => Some("Luigi"),
        "mario"           => Some("Mario"),
        "marth"           => Some("Marth"),
        "swordball"       => Some("Meta%20Knight"),
        "meta"            => Some("Meta%20Knight"),
        "metaknight"      => Some("Meta%20Knight"),
        "mk"              => Some("Meta%20Knight"),
        "ness"            => Some("Ness"),
        "olimar"          => Some("Olimar"),
        "oli"             => Some("Olimar"),
        "peach"           => Some("Peach"),
        "pikachu"         => Some("Pikachu"),
        "pika"            => Some("Pikachu"),
        "pit"             => Some("Pit"),
        "rob"             => Some("R.O.B"),
        "r.o.b"           => Some("R.O.B"),
        "samus"           => Some("Samus"),
        "sheik"           => Some("Sheik"),
        "solid"           => Some("Snake"),
        "solidsnake"      => Some("Snake"),
        "snake"           => Some("Snake"),
        "sonic"           => Some("Sonic"),
        "squirtle"        => Some("Squirtle"),
        "squirt"          => Some("Squirtle"),
        "toon"            => Some("Toon%20Link"),
        "toonlink"        => Some("Toon%20Link"),
        "tink"            => Some("Toon%20Link"),
        "tlink"           => Some("Toon%20Link"),
        "tl"              => Some("Toon%20Link"),
        "wario"           => Some("Wario"),
        "wario-man"       => Some("Wario-Man"),
        "warioman"        => Some("Wario-Man"),
        "wolf"            => Some("Wolf"),
        "yoshi"           => Some("Yoshi"),
        "yosh"            => Some("Yoshi"),
        "zelda"           => Some("Zelda"),
        "zero"            => Some("Zero%20Suit%20Samus"),
        "zerosuitsamus"   => Some("Zero%20Suit%20Samus"),
        "zss"             => Some("Zero%20Suit%20Samus"),
        _                 => None,
    }
}

#[rustfmt::skip]
pub fn pm(token: &str) -> Option<&'static str> {
    match token {
        "mewtwo" => Some("Mewtwo"),
        "mew2"   => Some("Mewtwo"),
        "m2"     => Some("Mewtwo"),
        "roy"    => Some("Roy"),
        _        => None,
    }
}

#[rustfmt::skip]
pub fn pplus(token: &str) -> Option<&'static str> {
    match token {
        "knuckles"      => Some("Knuckles"),
        "knucks"        => Some("Knuckles"),
        "knux"          => Some("Knuckles"),
        "fightingfreak" => Some("Knuckles"),
        "&"             => Some("Knuckles"),
        _               => None,
    }
}

#[rustfmt::skip]
// Sssssh
pub fn secret(token: &str) -> Option<&'static str> {
    match token {
        "knuckles"      => Some("Knuckles"),
        "knucks"        => Some("Knuckles"),
        "knux"          => Some("Knuckles"),
        "fightingfreak" => Some("Knuckles"),
        "&"             => Some("Knuckles"),
        "donald"        => Some("Donald"),
        "ronald"        => Some("Donald"),
        "mcdonald"      => Some("Donald"),
        _               => None,
    }
}

#[rustfmt::skip]
pub fn lxp(token: &str) -> Option<&'static str> {
    match token {
        "doctor"        => Some("DoctorMario"),
        "doctormario"   => Some("DoctorMario"),
        "doc"           => Some("DoctorMario"),
        "dr"            => Some("DoctorMario"),
        "drmario"       => Some("DoctorMario"),
        "dm"            => Some("DoctorMario"),
        "mage"          => Some("Ganon-Mage"),
        "mageganon"     => Some("Ganon-Mage"),
        "mg"            => Some("Ganon-Mage"),
        "geno"          => Some("Geno"),
        "lucina"        => Some("Lucina"),
        "metal"         => Some("MetalSonic"),
        "metalsonic"    => Some("MetalSonic"),
        "ms"            => Some("MetalSonic"),
        "pichu"         => Some("Pichu"),
        "ridley"        => Some("Ridley-Classic"),
        "classic"       => Some("Ridley-Classic"),
        "classicridley" => Some("Ridley-Classic"),
        "ridleyclassic" => Some("Ridley-Classic"),
        "modern"        => Some("Ridley-Modern"),
        "modernridley"  => Some("Ridley-Modern"),
        "ridleymodern"  => Some("Ridley-Modern"),
        "shadow"        => Some("Shadow"),
        "waluigi"       => Some("Waluigi"),
        "yl"            => Some("YoungLink"),
        "young"         => Some("YoungLink"),
        "younglink"     => Some("YoungLink"),
        "yink"          => Some("YoungLink"),
        "ylink"         => Some("YoungLink"),
        _               => None,
    }
}
