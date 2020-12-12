/*
* Copyright 2020 Mike Chambers
* https://github.com/mikechambers/dcli
*
* Permission is hereby granted, free of charge, to any person obtaining a copy of
* this software and associated documentation files (the "Software"), to deal in
* the Software without restriction, including without limitation the rights to
* use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
* of the Software, and to permit persons to whom the Software is furnished to do
* so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
* FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
* COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
* IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
* CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;
use std::str::FromStr;

//https://bungie-net.github.io/multi/schema_Destiny-HistoricalStats-Definitions-DestinyActivityModeType.html#schema_Destiny-HistoricalStats-Definitions-DestinyActivityModeType
#[derive(PartialEq, Clone, Copy, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum Mode {
    None = 0,
    Story = 2,
    Strike = 3,
    Raid = 4,
    AllPvP = 5,
    Patrol = 6,
    AllPvE = 7,
    Reserved9 = 9,
    Control = 10,
    Reserved11 = 11,
    Clash = 12,
    Reserved13 = 13,
    CrimsonDoubles = 15,
    Nightfall = 16,
    HeroicNightfall = 17,
    AllStrikes = 18,
    IronBanner = 19,
    Reserved20 = 20,
    Reserved21 = 21,
    Reserved22 = 22,
    Reserved24 = 24,
    AllMayhem = 25,
    Reserved26 = 26,
    Reserved27 = 27,
    Reserved28 = 28,
    Reserved29 = 29,
    Reserved30 = 30,
    Supremacy = 31,
    PrivateMatchesAll = 32,
    Survival = 37,
    Countdown = 38,
    TrialsOfTheNine = 39,
    Social = 40,
    TrialsCountdown = 41,
    TrialsSurvival = 42,
    IronBannerControl = 43,
    IronBannerClash = 44,
    IronBannerSupremacy = 45,
    ScoredNightfall = 46,
    ScoredHeroicNightfall = 47,
    Rumble = 48,
    AllDoubles = 49,
    Doubles = 50,
    PrivateMatchesClash = 51,
    PrivateMatchesControl = 52,
    PrivateMatchesSupremacy = 53,
    PrivateMatchesCountdown = 54,
    PrivateMatchesSurvival = 55,
    PrivateMatchesMayhem = 56,
    PrivateMatchesRumble = 57,
    HeroicAdventure = 58,
    Showdown = 59,
    Lockdown = 60,
    Scorched = 61,
    ScorchedTeam = 62,
    Gambit = 63,
    AllPvECompetitive = 64,
    Breakthrough = 65,
    BlackArmoryRun = 66,
    Salvage = 67,
    IronBannerSalvage = 68,
    PvPCompetitive = 69,
    PvPQuickplay = 70,
    ClashQuickplay = 71,
    ClashCompetitive = 72,
    ControlQuickplay = 73,
    ControlCompetitive = 74,
    GambitPrime = 75,
    Reckoning = 76,
    Menagerie = 77,
    VexOffensive = 78,
    NightmareHunt = 79,
    Elimination = 80,
    Momentum = 81,
    Dungeon = 82,
    Sundial = 83,
    TrialsOfOsiris = 84,
}

impl Mode {

    pub fn to_id(&self) -> u32 {
        *self as u32
    }

    pub fn is_gambit(&self) -> bool {
        *self == Mode::Gambit || *self == Mode::GambitPrime
    }

    pub fn is_nightfall(&self) -> bool {
        *self == Mode::Nightfall
            || *self == Mode::HeroicNightfall
            || *self == Mode::ScoredNightfall
            || *self == Mode::ScoredHeroicNightfall
    }

    pub fn is_crucible(&self) -> bool {
        *self == Mode::AllPvP
            || *self == Mode::Control
            || *self == Mode::Clash
            || *self == Mode::CrimsonDoubles
            || *self == Mode::IronBanner
            || *self == Mode::AllMayhem
            || *self == Mode::Supremacy
            || *self == Mode::Survival
            || *self == Mode::Countdown
            || *self == Mode::TrialsOfTheNine
            || *self == Mode::TrialsCountdown
            || *self == Mode::TrialsSurvival
            || *self == Mode::IronBannerControl
            || *self == Mode::IronBannerClash
            || *self == Mode::IronBannerSupremacy
            || *self == Mode::Rumble
            || *self == Mode::AllDoubles
            || *self == Mode::Doubles
            || *self == Mode::PrivateMatchesClash
            || *self == Mode::PrivateMatchesControl
            || *self == Mode::PrivateMatchesSupremacy
            || *self == Mode::PrivateMatchesCountdown
            || *self == Mode::PrivateMatchesSurvival
            || *self == Mode::PrivateMatchesMayhem
            || *self == Mode::PrivateMatchesRumble
            || *self == Mode::Showdown
            || *self == Mode::Lockdown
            || *self == Mode::Scorched
            || *self == Mode::ScorchedTeam
            || *self == Mode::Breakthrough
            || *self == Mode::Salvage
            || *self == Mode::IronBannerSalvage
            || *self == Mode::PvPCompetitive
            || *self == Mode::PvPQuickplay
            || *self == Mode::ClashQuickplay
            || *self == Mode::ClashCompetitive
            || *self == Mode::ControlQuickplay
            || *self == Mode::ControlCompetitive
            || *self == Mode::TrialsOfOsiris
    }
}

impl FromStr for Mode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //wrap in String so we can convert to lower case
        let s = String::from(s).to_lowercase();

        //get a slice to get a &str for the match
        match &s[..] {
            "none" => Ok(Mode::None),
            "story" => Ok(Mode::Story),
            "strike" => Ok(Mode::Strike),
            "raid" => Ok(Mode::Raid),
            "all_pvp" => Ok(Mode::AllPvP),
            "patrol" => Ok(Mode::Patrol),
            "all_pve" => Ok(Mode::AllPvE),
            //Reserved9 = 9,
            "control" => Ok(Mode::Control),
            //Reserved11 = 11,
            "clash" => Ok(Mode::Clash),
            //Reserved13 = 13,
            "crimsom_doubles" => Ok(Mode::CrimsonDoubles),
            "nightfall" => Ok(Mode::Nightfall),
            "heroic_nightfall" => Ok(Mode::HeroicNightfall),
            "all_strikes" => Ok(Mode::AllStrikes),
            "iron_banner" => Ok(Mode::IronBanner),
            //Reserved20 = 20,
            //Reserved21 = 21,
            //Reserved22 = 22,
            //Reserved24 = 24,
            "all_mayhem" => Ok(Mode::AllMayhem),
            //Reserved26 = 26,
            //Reserved27 = 27,
            //Reserved28 = 28,
            //Reserved29 = 29,
            //Reserved30 = 30,
            "supremacy" => Ok(Mode::Supremacy),
            //todo: think about renaming allprivatematches, or just privatematches (same with other
            //alls
            "all_private_matches" => Ok(Mode::PrivateMatchesAll),
            "survival" => Ok(Mode::Survival),
            "countdown" => Ok(Mode::Countdown),
            "trials_of_the_nine" => Ok(Mode::TrialsOfTheNine),
            "social" => Ok(Mode::Social),
            "trials_countdown" => Ok(Mode::TrialsCountdown),
            "trials_survival" => Ok(Mode::TrialsSurvival),
            "iron_banner_control" => Ok(Mode::IronBannerControl),
            "iron_banner_clash" => Ok(Mode::IronBannerClash),
            "iron_banner_supremacy" => Ok(Mode::IronBannerSupremacy),
            "scored_nightfall" => Ok(Mode::ScoredNightfall),
            "scored_heroic_nightfall" => Ok(Mode::ScoredHeroicNightfall),
            "rumble" => Ok(Mode::Rumble),
            "all_doubles" => Ok(Mode::AllDoubles),
            "doubles" => Ok(Mode::Doubles),
            "private_matches_clash" => Ok(Mode::PrivateMatchesClash),
            "private_matches_control" => Ok(Mode::PrivateMatchesControl),
            "private_matches_supremacy" => Ok(Mode::PrivateMatchesSupremacy),
            "private_matches_countdown" => Ok(Mode::PrivateMatchesCountdown),
            "private_matches_survival" => Ok(Mode::PrivateMatchesSurvival),
            "private_matches_mayhem" => Ok(Mode::PrivateMatchesMayhem),
            "private_matches_rumble" => Ok(Mode::PrivateMatchesRumble),
            "heroic_adventures" => Ok(Mode::HeroicAdventure),
            "showdown" => Ok(Mode::Showdown),
            "lockdown" => Ok(Mode::Lockdown),
            "scorched" => Ok(Mode::Scorched),
            "scorched_team" => Ok(Mode::ScorchedTeam),
            "gambit" => Ok(Mode::Gambit),
            //TODO: is this just all gambit?
            "all_pve_competitive" => Ok(Mode::AllPvECompetitive),
            "breakthrough" => Ok(Mode::Breakthrough),
            "black_armory_run" => Ok(Mode::BlackArmoryRun),
            "salvage" => Ok(Mode::Salvage),
            "iron_banner_salvage" => Ok(Mode::IronBannerSalvage),
            "pvp_competitive" => Ok(Mode::PvPCompetitive),
            "pvp_quickplay" => Ok(Mode::PvPQuickplay),
            "clash_quickplay" => Ok(Mode::ClashQuickplay),
            "clash_competitive" => Ok(Mode::ClashCompetitive),
            "control_quickplay" => Ok(Mode::ControlQuickplay),
            "control_competitive" => Ok(Mode::ControlCompetitive),
            "gambit_prime" => Ok(Mode::GambitPrime),
            "reckoning" => Ok(Mode::Reckoning),
            "menagerie" => Ok(Mode::Menagerie),
            "vex_offensive" => Ok(Mode::VexOffensive),
            "nightmare_hunt" => Ok(Mode::NightmareHunt),
            "elimination" => Ok(Mode::Elimination),
            "momentum" => Ok(Mode::Momentum),
            "dungeon" => Ok(Mode::Dungeon),
            "sundial" => Ok(Mode::Sundial),
            "trials_of_osiris" => Ok(Mode::TrialsOfOsiris),

            _ => Err("Unknown Mode type"),
        }
    }
}




impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            Mode::None => "None",
            Mode::Story => "Story",
            Mode::Strike => "Strike",
            Mode::Raid => "Raid",
            Mode::AllPvP => "All PvP",
            Mode::Patrol => "Patrol",
            Mode::AllPvE => "All PvE",
            Mode::Reserved9 => "Reserved9",
            Mode::Control => "Control",
            Mode::Reserved11 => "Reserved11",
            Mode::Clash => "Clash",
            Mode::Reserved13 => "Reserved13",
            Mode::CrimsonDoubles => "Crimson Doubles",
            Mode::Nightfall => "Nightfall",
            Mode::HeroicNightfall => "Heroic Nightfall",
            Mode::AllStrikes => "All Strikes",
            Mode::IronBanner => "Iron Banner",
            Mode::Reserved20 => "Reserved20",
            Mode::Reserved21 => "Reserved21",
            Mode::Reserved22 => "Reserved22",
            Mode::Reserved24 => "Reserved24",
            Mode::AllMayhem => "All Mayhem",
            Mode::Reserved26 => "Reserved26",
            Mode::Reserved27 => "Reserved27",
            Mode::Reserved28 => "Reserved28",
            Mode::Reserved29 => "Reserved29",
            Mode::Reserved30 => "Reserved30",
            Mode::Supremacy => "Supremacy",
            Mode::PrivateMatchesAll => "Private Matches All",
            Mode::Survival => "Survival",
            Mode::Countdown => "Countdown",
            Mode::TrialsOfTheNine => "Trials Of The Nine",
            Mode::Social => "Social",
            Mode::TrialsCountdown => "Trials Countdown",
            Mode::TrialsSurvival => "Trials Survival",
            Mode::IronBannerControl => "Iron Banner Control",
            Mode::IronBannerClash => "Iron Banner Clash",
            Mode::IronBannerSupremacy => "Iron Banner Supremacy",
            Mode::ScoredNightfall => "Scored Nightfall",
            Mode::ScoredHeroicNightfall => "Scored Heroic Nightfall",
            Mode::Rumble => "Rumble",
            Mode::AllDoubles => "All Doubles",
            Mode::Doubles => "Doubles",
            Mode::PrivateMatchesClash => "Private Matches Clash",
            Mode::PrivateMatchesControl => "Private Matches Control",
            Mode::PrivateMatchesSupremacy => "Private Matches Supremacy",
            Mode::PrivateMatchesCountdown => "Private Matches Countdown",
            Mode::PrivateMatchesSurvival => "Private Matches Survival",
            Mode::PrivateMatchesMayhem => "Private Matches Mayhem",
            Mode::PrivateMatchesRumble => "Private Matches Rumble",
            Mode::HeroicAdventure => "Heroic Adventure",
            Mode::Showdown => "Showdown",
            Mode::Lockdown => "Lockdown",
            Mode::Scorched => "Scorched",
            Mode::ScorchedTeam => "Scorched Team",
            Mode::Gambit => "Gambit",
            Mode::AllPvECompetitive => "All PvE Competitive",
            Mode::Breakthrough => "Breakthrough",
            Mode::BlackArmoryRun => "Black Armory Run",
            Mode::Salvage => "Salvage",
            Mode::IronBannerSalvage => "Iron BannerS alvage",
            Mode::PvPCompetitive => "PvP Competitive",
            Mode::PvPQuickplay => "PvP Quickplay",
            Mode::ClashQuickplay => "Clash Quickplay",
            Mode::ClashCompetitive => "Clash Competitive",
            Mode::ControlQuickplay => "Control Quickplay",
            Mode::ControlCompetitive => "Control Competitive",
            Mode::GambitPrime => "Gambit Prime",
            Mode::Reckoning => "Reckoning",
            Mode::Menagerie => "Menagerie",
            Mode::VexOffensive => "Vex Offensive",
            Mode::NightmareHunt => "Nightmare Hunt",
            Mode::Elimination => "Elimination",
            Mode::Momentum => "Momentum",
            Mode::Dungeon => "Dungeon",
            Mode::Sundial => "Sundial",
            Mode::TrialsOfOsiris => "Trials Of Osiris",
        };

        write!(f, "{}", out)
    }
}

