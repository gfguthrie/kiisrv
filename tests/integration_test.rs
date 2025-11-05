use kiisrv::kll::*;
use rstest::rstest;
use std::fs;

#[rstest]
#[case("Kira-Standard.json")]
fn parse_layout(#[case] json_file: &str) {
    let filename = format!("{}/{}", "layouts", json_file);
    println!("Parsing {}", filename);
    let config: KllConfig = {
        let contents = fs::read_to_string(filename).unwrap();
        serde_json::from_str(&contents).unwrap()
    };
    assert!(true);
}

#[rstest]
#[case("K-Type-Standard.json", "KType-Standard")]
#[case("K-Type-NoAnimations.json", "KType-NoAnimations")]
#[case("MD1.1-Alphabet.json", "MD1.1-Alphabet")]
#[case("MD1.1-AlphabetBlank.json", "MD1.1-AlphabetBlank")]
#[case("MD1.1-Hacker.json", "MD1.1-Hacker")]
#[case("MD1.1-HackerBlank.json", "MD1.1-HackerBlank")]
#[case("MD1.1-Standard.json", "MD1.1-Standard")]
#[case("MD1.1-StandardBlank.json", "MD1.1-StandardBlank")]
#[case("MD1-Hacker.json", "MD1-Hacker")]
#[case("MD1-HackerBlank.json", "MD1-HackerBlank")]
#[case("MD1-Standard.json", "MD1-Standard")]
#[case("MD1-StandardBlank.json", "MD1-StandardBlank")]
#[case("MDErgo1-Blank.json", "MDErgo1-Blank")]
#[case("MDErgo1-Default.json", "MDErgo1-Default")]
#[case("WhiteFox-Aria.json", "WhiteFox-Aria")]
#[case("WhiteFox-Iso.json", "WhiteFox-Iso")]
#[case("WhiteFox-JackofAllTrades.json", "WhiteFox-JackofAllTrades")]
#[case("WhiteFox-TheTrueFox.json", "WhiteFox-TheTrueFox")]
#[case("WhiteFox-Vanilla.json", "WhiteFox-Vanilla")]
#[case("WhiteFox-Winkeyless.json", "WhiteFox-Winkeyless")]
fn generate_kll_latest(#[case] json_file: &str, #[case] kll_dir: &str) {
    let filename = format!("{}/{}", "layouts", json_file);
    println!("Parsing {}", filename);
    let config: KllConfig = {
        let contents = fs::read_to_string(filename).unwrap();
        serde_json::from_str(&contents).unwrap()
    };

    let files = generate_kll(&config, false);
    for file in files {
        let kll_file = format!("{}/{}/{}", "tests/web_latest", kll_dir, file.name);
        println!("Comparing to {}", kll_file);
        let kll = fs::read_to_string(kll_file).unwrap();
        assert_eq!(file.content, kll);
    }
}

#[rstest]
#[case("K-Type-Standard.json", "KType-Standard")]
#[case("K-Type-NoAnimations.json", "KType-NoAnimations")]
#[case("MD1.1-Alphabet.json", "MD1.1-Alphabet")]
#[case("MD1.1-AlphabetBlank.json", "MD1.1-AlphabetBlank")]
#[case("MD1.1-Hacker.json", "MD1.1-Hacker")]
#[case("MD1.1-HackerBlank.json", "MD1.1-HackerBlank")]
#[case("MD1.1-Standard.json", "MD1.1-Standard")]
#[case("MD1.1-StandardBlank.json", "MD1.1-StandardBlank")]
#[case("MD1-Hacker.json", "MD1-Hacker")]
#[case("MD1-HackerBlank.json", "MD1-HackerBlank")]
#[case("MD1-Standard.json", "MD1-Standard")]
#[case("MD1-StandardBlank.json", "MD1-StandardBlank")]
#[case("MDErgo1-Blank.json", "MDErgo1-Blank")]
#[case("MDErgo1-Default.json", "MDErgo1-Default")]
#[case("WhiteFox-AriaBlank.json", "WhiteFox-AriaBlank")]
#[case("WhiteFox-IsoBlank.json", "WhiteFox-IsoBlank")]
#[case("WhiteFox-JackBlank.json", "WhiteFox-JackBlank")]
#[case("WhiteFox-TrueFoxBlank.json", "WhiteFox-TrueFoxBlank")]
#[case("WhiteFox-VanillaBlank.json", "WhiteFox-VanillaBlank")]
#[case("WhiteFox-WinkeylessBlank.json", "WhiteFox-WinkeylessBlank")]
fn generate_kll_lts(#[case] json_file: &str, #[case] kll_dir: &str) {
    let filename = format!("{}/{}", "layouts", json_file);
    println!("Parsing {}", filename);
    let config: KllConfig = {
        let contents = fs::read_to_string(filename).unwrap();
        serde_json::from_str(&contents).unwrap()
    };

    let files = generate_kll(&config, true);
    for file in files {
        let kll_file = format!("{}/{}/{}", "tests/web_lts", kll_dir, file.name);
        println!("Comparing to {}", kll_file);
        let kll = fs::read_to_string(kll_file).unwrap();
        assert_eq!(file.content, kll);
    }
}


