use pest::Parser;
use pest_derive::Parser;
use anyhow::anyhow;


#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Command;

fn main() -> anyhow::Result< () > {
    Ok(())
}

// UNIT TESTS

#[cfg(test)]
mod tests {

    use super::*;

    // test coordinates rule
    #[test]
    fn test_coordinates() -> anyhow::Result< () > {

        let res = Command::parse(Rule::coordinates, "(3,4)")?.next().ok_or_else( || anyhow!( "Failed to parse coordinates" ) )?;
        assert_eq!( res.as_str(), "(3,4)" );
        assert_eq!( res.as_span().start(), 0 );
        assert_eq!( res.as_span().end(), 5 );

        assert!(Command::parse(Rule::coordinates, "5g").is_err());
        assert!(Command::parse(Rule::coordinates, "").is_err());
        assert!(Command::parse(Rule::coordinates, "(2, 5)").is_err());
        assert!(Command::parse(Rule::coordinates, "(,4)").is_err());

        Ok(())
    }

    // test "new node" rule
    #[test]
    fn test_new_node() -> anyhow::Result< () > {

        let res = Command::parse(Rule::new_node, "NEW NODE B1 (3,4)")?.next().ok_or_else( || anyhow!( "Failed to parse new node command" ) )?;
        assert_eq!( res.as_str(), "NEW NODE B1 (3,4)" );
        assert_eq!( res.as_span().start(), 0 );
        assert_eq!( res.as_span().end(), 17 );

        assert!(Command::parse(Rule::new_node, "CREATE NODE B1 (3,4)").is_err());
        assert!(Command::parse(Rule::new_node, "NEW node B1 (3,4)").is_err());
        assert!(Command::parse(Rule::new_node, "new node B1 (3,4)").is_err());
        assert!(Command::parse(Rule::new_node, "NEW  NODE B1 (3,4)").is_err());
        assert!(Command::parse(Rule::new_node, "NEW NODE B1 (3,)").is_err());
        assert!(Command::parse(Rule::new_node, "NEW NODE (3,4)").is_err());

        Ok(())
    }
}


