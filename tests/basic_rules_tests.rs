use pest::Parser;
use anyhow::anyhow;

use graph_commands_parser::*;

// cargo test --test basic_rules_tests

// test name 
#[test]
fn test_names() -> anyhow::Result< () > {

    assert!(Command::parse(Rule::name, "A").is_ok());
    assert!(Command::parse(Rule::name, "BB").is_ok());
    assert!(Command::parse(Rule::name, "A2").is_ok());
    assert!(Command::parse(Rule::name, "Aq").is_ok());

    assert!(Command::parse(Rule::name, "5fg").is_err());
    assert!(Command::parse(Rule::name, "2V").is_err());
    assert!(Command::parse(Rule::name, "-").is_err());
    assert!(Command::parse(Rule::name, " Bd").is_err());

    Ok(())
}

// test number 
#[test]
fn test_number() -> anyhow::Result< () > {

    let res = Command::parse(Rule::number, "0,45")?.next().ok_or_else( || anyhow!( "Failed to parse number" ) )?;
    let res2 = Command::parse(Rule::number, "0.45")?.next().ok_or_else( || anyhow!( "Failed to parse number" ) )?;

    // should not be equall becuase parser did not consume the whole string due to "," invalid separator
    assert_ne!(res.as_str(), "0,45");

    assert_eq!( res2.as_str(), "0.45" );

    assert!(Command::parse(Rule::number, "1").is_ok());
    assert!(Command::parse(Rule::number, "46743").is_ok());
    assert!(Command::parse(Rule::number, "34.68").is_ok());
    assert!(Command::parse(Rule::number, "-0.22").is_ok());

    assert!(Command::parse(Rule::number, "--45").is_err());
    assert!(Command::parse(Rule::number, "").is_err());
    assert!(Command::parse(Rule::number, "  ").is_err());
    assert!(Command::parse(Rule::number, ".2").is_err());

    Ok(())
}

// test coordinates
#[test]
fn test_coordinates() -> anyhow::Result< () > {

    assert!(Command::parse(Rule::coordinates, "(2,3)").is_ok());
    assert!(Command::parse(Rule::coordinates, "(4,-5.5)").is_ok());

    assert!(Command::parse(Rule::coordinates, "5g").is_err());
    assert!(Command::parse(Rule::coordinates, "").is_err());
    assert!(Command::parse(Rule::coordinates, "(2, 5)").is_err());
    assert!(Command::parse(Rule::coordinates, "(,4)").is_err());

    Ok(())
}

// test new lines
#[test]
fn test_new_lines() -> anyhow::Result< () > {

    assert!(Command::parse(Rule::NEWLINE, "\n").is_ok());
    assert!(Command::parse(Rule::NEWLINE, "\r\n").is_ok());

    assert!(Command::parse(Rule::NEWLINE, "12").is_err());
    assert!(Command::parse(Rule::NEWLINE, "").is_err());
    assert!(Command::parse(Rule::NEWLINE, "     ").is_err());
    assert!(Command::parse(Rule::NEWLINE, "     
    ").is_err());

    Ok(())
}



