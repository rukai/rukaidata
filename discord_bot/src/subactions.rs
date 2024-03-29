#[rustfmt::skip]
pub fn subactions(tokens: &[&str], character: &str) -> Vec<&'static str> {
    // TODO: This should be made a vector to contain all the subactions related to the action. (multiple jabs, smash attack startup/attack)
    // TODO: Manually handle character specific stuff such as jabs, glides, etc
    let mut subactions = vec!();

    // common movement
    if tokens.contains(&"dash")                              { subactions = vec!("Dash") }
    if tokens.contains(&"run")                               { subactions = vec!("Run") }
    if tokens.contains(&"teeter")                            { subactions = vec!("OttottoWait") }
    if tokens.contains(&"crouch")                            { subactions = vec!("SquatWait") }
    if tokens.contains(&"idle")                              { subactions = vec!("Wait1") }
    if tokens.contains(&"jump") && tokens.contains(&"squat") { subactions = vec!("JumpSquat") }
    if tokens.contains(&"jumpsquat")                         { subactions = vec!("JumpSquat") }

    // jabs
    if tokens.contains(&"jab") { subactions = vec!("Attack11") }

    // dash attack
    if tokens.contains(&"dash") && tokens.contains(&"attack") { subactions = vec!("AttackDash") }
    if tokens.contains(&"dashattack")                         { subactions = vec!("AttackDash") }

    // grabs
    if tokens.contains(&"grab")                              { subactions = vec!("Catch") }
    if tokens.contains(&"dash")  && tokens.contains(&"grab") { subactions = vec!("CatchDash") }
    if tokens.contains(&"dashgrab")                          { subactions = vec!("CatchDash") }
    if tokens.contains(&"pivot") && tokens.contains(&"grab") { subactions = vec!("CatchTurn") }
    if tokens.contains(&"pivotgrab")                         { subactions = vec!("CatchTurn") }
    if tokens.contains(&"turn")  && tokens.contains(&"grab") { subactions = vec!("CatchTurn") }
    if tokens.contains(&"turngrab")                          { subactions = vec!("CatchTurn") }
    if tokens.contains(&"pummel")                            { subactions = vec!("CatchAttack") }

    //throws
    if tokens.contains(&"up")      && tokens.contains(&"throw") { subactions = vec!("ThrowHi") }
    if tokens.contains(&"upthrow")                              { subactions = vec!("ThrowHi") }
    if tokens.contains(&"uthrow")                               { subactions = vec!("ThrowHi") }
    if tokens.contains(&"down")    && tokens.contains(&"throw") { subactions = vec!("ThrowLw") }
    if tokens.contains(&"downthrow")                            { subactions = vec!("ThrowLw") }
    if tokens.contains(&"dthrow")                               { subactions = vec!("ThrowLw") }
    if tokens.contains(&"back")    && tokens.contains(&"throw") { subactions = vec!("ThrowB") }
    if tokens.contains(&"backthrow")                            { subactions = vec!("ThrowB") }
    if tokens.contains(&"bthrow")                               { subactions = vec!("ThrowB") }
    if tokens.contains(&"forward") && tokens.contains(&"throw") { subactions = vec!("ThrowF") }
    if tokens.contains(&"forwardthrow")                         { subactions = vec!("ThrowF") }
    if tokens.contains(&"fthrow")                               { subactions = vec!("ThrowF") }

    // tilts
    if tokens.contains(&"up")      && tokens.contains(&"tilt") { subactions = vec!("AttackHi3") }
    if tokens.contains(&"uptilt")                              { subactions = vec!("AttackHi3") }
    if tokens.contains(&"utilt")                               { subactions = vec!("AttackHi3") }
    if tokens.contains(&"down")    && tokens.contains(&"tilt") { subactions = vec!("AttackLw3") }
    if tokens.contains(&"downtilt")                            { subactions = vec!("AttackLw3") }
    if tokens.contains(&"dtilt")                               { subactions = vec!("AttackLw3") }
    if tokens.contains(&"forward") && tokens.contains(&"tilt") { subactions = vec!("AttackS3S") }
    if tokens.contains(&"forwardtilt")                         { subactions = vec!("AttackS3S") }
    if tokens.contains(&"ftilt")                               { subactions = vec!("AttackS3S") }
    if tokens.contains(&"side")    && tokens.contains(&"tilt") { subactions = vec!("AttackS3S") }
    if tokens.contains(&"sidetilt")                            { subactions = vec!("AttackS3S") }
    if tokens.contains(&"stilt")                               { subactions = vec!("AttackS3S") }

    // ledge getup
    let ledge = tokens.contains(&"ledge") || tokens.contains(&"edge") || tokens.contains(&"cliff");
    if ledge && tokens.contains(&"attack") && tokens.contains(&"slow")  { subactions = vec!("CliffAttackSlow") }
    if ledge && tokens.contains(&"attack") && tokens.contains(&"quick") { subactions = vec!("CliffAttackQuick") }
    if ledge && tokens.contains(&"roll")   && tokens.contains(&"slow")  { subactions = vec!("CliffEscapeSlow") }
    if ledge && tokens.contains(&"roll")   && tokens.contains(&"quick") { subactions = vec!("CliffEscapeQuick") }
    if ledge && tokens.contains(&"getup")  && tokens.contains(&"slow")  { subactions = vec!("CliffClimbSlow") }
    if ledge && tokens.contains(&"getup")  && tokens.contains(&"quick") { subactions = vec!("CliffClimbQuick") }

    // getup
    let facedown = tokens.contains(&"facedown") || tokens.contains(&"down") || tokens.contains(&"d");
    if tokens.contains(&"getup") && tokens.contains(&"attack")             { subactions = vec!("DownAttackU") }
    if tokens.contains(&"getup") && tokens.contains(&"attack") && facedown { subactions = vec!("DownAttackD") }
    if tokens.contains(&"getup") && tokens.contains(&"stand")              { subactions = vec!("DownStandU") }
    if tokens.contains(&"getup") && tokens.contains(&"stand")  && facedown { subactions = vec!("DownStandD") }

    // trip
    if tokens.contains(&"trip") || tokens.contains(&"slip")   { subactions = vec!("Slip") }
    if tokens.contains(&"trip") && tokens.contains(&"attack") { subactions = vec!("DownAttackU") }
    if tokens.contains(&"trip") && tokens.contains(&"stand")  { subactions = vec!("DownStandU") }

    // escape
    if tokens.contains(&"spotdodge")                                { subactions = vec!("EscapeN") }
    if tokens.contains(&"spot")    && tokens.contains(&"dodge")     { subactions = vec!("EscapeN") }
    if tokens.contains(&"airdodge")                                 { subactions = vec!("EscapeAir") }
    if tokens.contains(&"air")     && tokens.contains(&"dodge")     { subactions = vec!("EscapeAir") }
    if tokens.contains(&"roll")    && tokens.contains(&"forward")   { subactions = vec!("EscapeF") }
    if tokens.contains(&"roll")    && tokens.contains(&"forwards")  { subactions = vec!("EscapeF") }
    if tokens.contains(&"roll")    && tokens.contains(&"backward")  { subactions = vec!("EscapeB") }
    if tokens.contains(&"roll")    && tokens.contains(&"backwards") { subactions = vec!("EscapeB") }
    if tokens.contains(&"roll")    && tokens.contains(&"back")      { subactions = vec!("EscapeB") }

    // yeet
    if tokens.contains(&"yeet") && "Ness" == character {
        subactions = vec!("ThrowB");
    }

    // crawl attack
    if (tokens.contains(&"crawl") && tokens.contains(&"attack")) ||
        (tokens.contains(&"crawl") && tokens.contains(&"tilt"))  ||
        tokens.contains(&"ctilt") {
        match character {
            "Lucario"  => subactions = vec!("AttackSquat"),
            "Squirtle" => subactions = vec!("AttackSquat"),
            "Snake"    => subactions = vec!("AttackLwShank"),
            "Samus"    => subactions = vec!("SpecialSDash"),
            _ => { }
        }
    }

    // Smashes
    if tokens.contains(&"up")      && tokens.contains(&"smash") { subactions = vec!("AttackHi4Start", "AttackHi4") }
    if tokens.contains(&"upsmash")                              { subactions = vec!("AttackHi4Start", "AttackHi4") }
    if tokens.contains(&"usmash")                               { subactions = vec!("AttackHi4Start", "AttackHi4") }
    if tokens.contains(&"down")    && tokens.contains(&"smash") { subactions = vec!("AttackLw4Start", "AttackLw4") }
    if tokens.contains(&"downsmash")                            { subactions = vec!("AttackLw4Start", "AttackLw4") }
    if tokens.contains(&"dsmash")                               { subactions = vec!("AttackLw4Start", "AttackLw4") }
    if tokens.contains(&"forward") && tokens.contains(&"smash") { subactions = vec!("AttackS4Start", "AttackS4S") }
    if tokens.contains(&"forwardsmash")                         { subactions = vec!("AttackS4Start", "AttackS4S") }
    if tokens.contains(&"fsmash")                               { subactions = vec!("AttackS4Start", "AttackS4S") }
    if tokens.contains(&"side")    && tokens.contains(&"smash") { subactions = vec!("AttackS4Start", "AttackS4S") }
    if tokens.contains(&"sidesmash")                            { subactions = vec!("AttackS4Start", "AttackS4S") }
    if tokens.contains(&"ssmash")                               { subactions = vec!("AttackS4Start", "AttackS4S") }

    // aerials
    if tokens.contains(&"up")      && tokens.contains(&"air") { subactions = vec!("AttackAirHi") }
    if tokens.contains(&"upair")                              { subactions = vec!("AttackAirHi") }
    if tokens.contains(&"uair")                               { subactions = vec!("AttackAirHi") }
    if tokens.contains(&"down")    && tokens.contains(&"air") { subactions = vec!("AttackAirLw") }
    if tokens.contains(&"downair")                            { subactions = vec!("AttackAirLw") }
    if tokens.contains(&"dair")                               { subactions = vec!("AttackAirLw") }
    if tokens.contains(&"forward") && tokens.contains(&"air") { subactions = vec!("AttackAirF") }
    if tokens.contains(&"forwardair")                         { subactions = vec!("AttackAirF") }
    if tokens.contains(&"fair")                               { subactions = vec!("AttackAirF") }
    if tokens.contains(&"unfair")                             { subactions = vec!("AttackAirF") }
    if tokens.contains(&"back")    && tokens.contains(&"air") { subactions = vec!("AttackAirB") }
    if tokens.contains(&"backair")                            { subactions = vec!("AttackAirB") }
    if tokens.contains(&"bair")                               { subactions = vec!("AttackAirB") }
    if tokens.contains(&"neutral") && tokens.contains(&"air") { subactions = vec!("AttackAirN") }
    if tokens.contains(&"neutralair")                         { subactions = vec!("AttackAirN") }
    if tokens.contains(&"nair")                               { subactions = vec!("AttackAirN") }

    // specials
    if tokens.contains(&"u")       && tokens.contains(&"special") { subactions = vec!("SpecialHi") }
    if tokens.contains(&"up")      && tokens.contains(&"special") { subactions = vec!("SpecialHi") }
    if tokens.contains(&"up")      && tokens.contains(&"b")       { subactions = vec!("SpecialHi") }
    if tokens.contains(&"upspecial")                              { subactions = vec!("SpecialHi") }
    if tokens.contains(&"uspecial")                               { subactions = vec!("SpecialHi") }
    if tokens.contains(&"upb")                                    { subactions = vec!("SpecialHi") }
    if tokens.contains(&"d")       && tokens.contains(&"special") { subactions = vec!("SpecialLw") }
    if tokens.contains(&"down")    && tokens.contains(&"special") { subactions = vec!("SpecialLw") }
    if tokens.contains(&"down")    && tokens.contains(&"b")       { subactions = vec!("SpecialLw") }
    if tokens.contains(&"downspecial")                            { subactions = vec!("SpecialLw") }
    if tokens.contains(&"dspecial")                               { subactions = vec!("SpecialLw") }
    if tokens.contains(&"downb")                                  { subactions = vec!("SpecialLw") }
    if tokens.contains(&"n")       && tokens.contains(&"special") { subactions = vec!("SpecialN") }
    if tokens.contains(&"neutral") && tokens.contains(&"special") { subactions = vec!("SpecialN") }
    if tokens.contains(&"neutral") && tokens.contains(&"b")       { subactions = vec!("SpecialN") }
    if tokens.contains(&"neutralspecial")                         { subactions = vec!("SpecialN") }
    if tokens.contains(&"nspecial")                               { subactions = vec!("SpecialN") }
    if tokens.contains(&"neutralb")                               { subactions = vec!("SpecialN") }
    if tokens.contains(&"f")       && tokens.contains(&"special") { subactions = vec!("SpecialS") }
    if tokens.contains(&"forward") && tokens.contains(&"special") { subactions = vec!("SpecialS") }
    if tokens.contains(&"forward") && tokens.contains(&"b")       { subactions = vec!("SpecialS") }
    if tokens.contains(&"forwardspecial")                         { subactions = vec!("SpecialS") }
    if tokens.contains(&"fspecial")                               { subactions = vec!("SpecialS") }
    if tokens.contains(&"forwardb")                               { subactions = vec!("SpecialS") }
    if tokens.contains(&"s")       && tokens.contains(&"special") { subactions = vec!("SpecialS") }
    if tokens.contains(&"side")    && tokens.contains(&"special") { subactions = vec!("SpecialS") }
    if tokens.contains(&"side")    && tokens.contains(&"b")       { subactions = vec!("SpecialS") }
    if tokens.contains(&"sidespecial")                            { subactions = vec!("SpecialS") }
    if tokens.contains(&"sspecial")                               { subactions = vec!("SpecialS") }
    if tokens.contains(&"sideb")                                  { subactions = vec!("SpecialS") }

    // specials air
    if tokens.contains(&"air") && tokens.contains(&"u")       && tokens.contains(&"special") { subactions = vec!("SpecialAirHi") }
    if tokens.contains(&"air") && tokens.contains(&"up")      && tokens.contains(&"special") { subactions = vec!("SpecialAirHi") }
    if tokens.contains(&"air") && tokens.contains(&"up")      && tokens.contains(&"b")       { subactions = vec!("SpecialAirHi") }
    if tokens.contains(&"air") && tokens.contains(&"upspecial")                              { subactions = vec!("SpecialAirHi") }
    if tokens.contains(&"air") && tokens.contains(&"uspecial")                               { subactions = vec!("SpecialAirHi") }
    if tokens.contains(&"air") && tokens.contains(&"upb")                                    { subactions = vec!("SpecialAirHi") }
    if tokens.contains(&"air") && tokens.contains(&"d")       && tokens.contains(&"special") { subactions = vec!("SpecialAirLw") }
    if tokens.contains(&"air") && tokens.contains(&"down")    && tokens.contains(&"special") { subactions = vec!("SpecialAirLw") }
    if tokens.contains(&"air") && tokens.contains(&"down")    && tokens.contains(&"b")       { subactions = vec!("SpecialAirLw") }
    if tokens.contains(&"air") && tokens.contains(&"downspecial")                            { subactions = vec!("SpecialAirLw") }
    if tokens.contains(&"air") && tokens.contains(&"dspecial")                               { subactions = vec!("SpecialAirLw") }
    if tokens.contains(&"air") && tokens.contains(&"downb")                                  { subactions = vec!("SpecialAirLw") }
    if tokens.contains(&"air") && tokens.contains(&"n")       && tokens.contains(&"special") { subactions = vec!("SpecialAirN") }
    if tokens.contains(&"air") && tokens.contains(&"neutral") && tokens.contains(&"special") { subactions = vec!("SpecialAirN") }
    if tokens.contains(&"air") && tokens.contains(&"neutral") && tokens.contains(&"b")       { subactions = vec!("SpecialAirN") }
    if tokens.contains(&"air") && tokens.contains(&"neutralspecial")                         { subactions = vec!("SpecialAirN") }
    if tokens.contains(&"air") && tokens.contains(&"nspecial")                               { subactions = vec!("SpecialAirN") }
    if tokens.contains(&"air") && tokens.contains(&"neutralb")                               { subactions = vec!("SpecialAirN") }
    if tokens.contains(&"air") && tokens.contains(&"f")       && tokens.contains(&"special") { subactions = vec!("SpecialAirS") }
    if tokens.contains(&"air") && tokens.contains(&"forward") && tokens.contains(&"special") { subactions = vec!("SpecialAirS") }
    if tokens.contains(&"air") && tokens.contains(&"forward") && tokens.contains(&"b")       { subactions = vec!("SpecialAirS") }
    if tokens.contains(&"air") && tokens.contains(&"forwardspecial")                         { subactions = vec!("SpecialAirS") }
    if tokens.contains(&"air") && tokens.contains(&"fspecial")                               { subactions = vec!("SpecialAirS") }
    if tokens.contains(&"air") && tokens.contains(&"forwardb")                               { subactions = vec!("SpecialAirS") }
    if tokens.contains(&"air") && tokens.contains(&"s")       && tokens.contains(&"special") { subactions = vec!("SpecialAirS") }
    if tokens.contains(&"air") && tokens.contains(&"side")    && tokens.contains(&"special") { subactions = vec!("SpecialAirS") }
    if tokens.contains(&"air") && tokens.contains(&"side")    && tokens.contains(&"b")       { subactions = vec!("SpecialAirS") }
    if tokens.contains(&"air") && tokens.contains(&"sidespecial")                            { subactions = vec!("SpecialAirS") }
    if tokens.contains(&"air") && tokens.contains(&"sspecial")                               { subactions = vec!("SpecialAirS") }
    if tokens.contains(&"air") && tokens.contains(&"sideb")                                  { subactions = vec!("SpecialAirS") }

    // taunts
    if tokens.contains(&"utaunt") { subactions = vec!("AppealHiR") }
    if tokens.contains(&"dtaunt") { subactions = vec!("AppealLwR") }
    if tokens.contains(&"staunt") { subactions = vec!("AppealS") }
    if tokens.contains(&"up")   && tokens.contains(&"taunt") { subactions = vec!("AppealHi") }
    if tokens.contains(&"down") && tokens.contains(&"taunt") { subactions = vec!("AppealLw") }
    if tokens.contains(&"side") && tokens.contains(&"taunt") { subactions = vec!("AppealS") }
    if tokens.contains(&"lose")                              { subactions = vec!("Lose") }
    if (tokens.contains(&"1") && tokens.contains(&"win")) || tokens.contains(&"win1") { subactions = vec!("Win1") }
    if (tokens.contains(&"2") && tokens.contains(&"win")) || tokens.contains(&"win2") { subactions = vec!("Win2") }
    if (tokens.contains(&"3") && tokens.contains(&"win")) || tokens.contains(&"win3") { subactions = vec!("Win3") }

    subactions
}
