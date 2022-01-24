use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Import {
    pub path: String,
}

impl FromStr for Import {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split_whitespace();

        let _position = splitted
            .position(|x| x == "from")
            .ok_or(())?;

         let path = splitted
            .next()
            .ok_or(())?
            .trim_end_matches(';')
            .trim_matches('\'')
            .to_string();

        Ok(Import{ path })
    }
}

#[cfg(test)]
mod tests {
    use crate::import::Import;
    use std::str::FromStr;

    #[test]
    fn it_works() {
        let line = "import { default as AdminAction } from './services/admin/action';";

        let topic = Import::from_str(line);
        
        assert_eq!(topic, Ok(Import { path: String::from("./services/admin/action") }))
    }
}