use pest::Parser;
use anyhow::anyhow;

use graph_commands_parser::*;

// cargo test --test main_commands_tests

// new node command
#[test]
fn test_new_node_command() -> anyhow::Result< () > {

    assert!(Command::parse(Rule::new_node, "NEW NODE B1 (3,4)").is_ok());
    assert!(Command::parse(Rule::new_node, "NEW NODE DD (3,4) A1 (1,1)").is_ok());
    assert!(Command::parse(Rule::new_node, "NEW NODE A (3.5,4)").is_ok());
    assert!(Command::parse(Rule::new_node, "NEW NODE B1 (3,-4.8)").is_ok());

    assert!(Command::parse(Rule::new_node, "CREATE NODE B1 (3,4)").is_err());
    assert!(Command::parse(Rule::new_node, "NEW node B1 (3,4)").is_err());
    assert!(Command::parse(Rule::new_node, "new node B1 (3,4)").is_err());
    assert!(Command::parse(Rule::new_node, "NEW  NODE B1 (3,4)").is_err());
    assert!(Command::parse(Rule::new_node, "NEW NODE B1 (3,)").is_err());
    assert!(Command::parse(Rule::new_node, "NEW NODE (3,4)").is_err());

    Ok(())
}

// new edge command
#[test]
fn test_new_edge_command() -> anyhow::Result< () > {

    assert!(Command::parse(Rule::new_edge, "NEW EDGE A1-B2").is_ok());
    assert!(Command::parse(Rule::new_edge, "NEW EDGE A1-B2 D1-K B-C").is_ok());

    assert!(Command::parse(Rule::new_edge, "NEW EDGE 1-2").is_err());
    assert!(Command::parse(Rule::new_edge, "NEW EDGE A1A-B2").is_err());
    assert!(Command::parse(Rule::new_edge, "NEW EDGE A1").is_err());
    assert!(Command::parse(Rule::new_edge, "OLD EDGE B-C").is_err());

    Ok(())
}

// delete node command 
#[test]
fn test_delete_node_command() -> anyhow::Result< () > {

    let res = Command::parse(Rule::delete_node, "DELETE NODE A1 BCV")?.next().ok_or_else( || anyhow!( "Failed to parse delete node command" ) )?;
    let res2 = Command::parse(Rule::delete_node, "DELETE NODE ALL")?.next().ok_or_else( || anyhow!( "Failed to parse delete node command" ) )?;
    let res3 = Command::parse(Rule::delete_node, "DELETE NODE A1-A2")?.next().ok_or_else( || anyhow!( "Failed to parse delete node command" ) )?;

    // should not be equall becuase parser did not consume the whole string due to invalid names
    assert_ne!(res.as_str(), "DELETE NODE A1 BCV");
    assert_ne!(res2.as_str(), "DELETE NODE ALL");
    assert_ne!(res3.as_str(), "DELETE NODE A1-A2");

    assert!(Command::parse(Rule::delete_node, "DELETE NODE A1").is_ok());
    assert!(Command::parse(Rule::delete_node, "DELETE NODE A1 B1 C1").is_ok());

    Ok(())
}

// delete edge command  
#[test]
fn test_delete_edge_command() -> anyhow::Result< () > {

    let res1 = Command::parse(Rule::delete_edge, "DELETE EDGE A1-B2-B3")?.next().ok_or_else( || anyhow!( "Failed to parse delete edge command" ) )?;

    // should not be equall becuase parser did not consume the whole string due to invalid names
    assert_ne!(res1.as_str(), "DELETE EDGE A1-B2-B3");

    assert!(Command::parse(Rule::delete_edge, "DELETE EDGE A1-B2").is_ok());
    assert!(Command::parse(Rule::delete_edge, "DELETE EDGE A1-B2 b-n").is_ok());
    assert!(Command::parse(Rule::delete_edge, "DELETE EDGE A1-B2 W1-W2 W3-W4").is_ok());

    assert!(Command::parse(Rule::delete_edge, "DELETE EDGE A1").is_err());
    assert!(Command::parse(Rule::delete_edge, "DELETE EDGE A1/B2").is_err());

    Ok(())
}

// create graph path 
#[test]
fn test_create_path_command() -> anyhow::Result< () > {

    assert!(Command::parse(Rule::new_path, "CONNECT PATH A1 -> B2 -> K4").is_ok());
    assert!(Command::parse(Rule::new_path, "CONNECT PATH A1 -> B2 -> K4 -> L1").is_ok());
    assert!(Command::parse(Rule::new_path, "CONNECT PATH A1 -> B2 -> K4 -> L1 -> A1").is_ok());

    // at least 3 nodes should be connected
    assert!(Command::parse(Rule::new_path, "CONNECT PATH A1 -> B2").is_err());
    assert!(Command::parse(Rule::new_path, "CONNECT PATH A1 -> B2 <- K4").is_err());

    Ok(())
}

// file rule 
#[test]
fn test_file_rule() -> anyhow::Result< () > {

    let input1 = "NEW NODE B3 (1,1)";

    let input2 = "NEW NODE A1 (1,1)\nNEW NODE A2 (2,2)\nNEW EDGE A1-A2\nDELETE EDGE A1-A2\n";

    assert!(Command::parse(Rule::file, input1).is_ok());
    assert!(Command::parse(Rule::file, input2).is_ok());

    // each command should start at the start of line
    let input3 = "  NEW NODE B3 (1,1)\nDELETE NODE B3";

    // ; - not allowed
    let input4 = "CREATE NODE A1 (2,2);\nCREATE NODE B2 (3,3);\nCONNECT PATH A1 -> B2 -> K4 -> L1;";

    // invalid command
    let input5 = "NEW NODE B3 (1,1)\nCREATE NODE B1 (2,2)";

    assert!(Command::parse(Rule::file, input3).is_err());
    assert!(Command::parse(Rule::file, input4).is_err());
    assert!(Command::parse(Rule::file, input5).is_err());

    Ok(())
}





