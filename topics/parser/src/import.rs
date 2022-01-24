use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Import {
    pub path: String,
}

impl Import {
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_file(&self) -> String {
        String::from("index.ts")
    }
}

impl FromStr for Import {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split_whitespace();

        let _position = splitted.position(|x| x == "from").ok_or(())?;

        let path = splitted
            .next()
            .ok_or(())?
            .trim_end_matches(';')
            .trim_matches('\'')
            .to_string();

        if path.starts_with("@") {
            Err(())
        } else {
            Ok(Import { path })
        }
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

        assert_eq!(
            topic,
            Ok(Import {
                path: String::from("./services/admin/action")
            })
        )
    }
}
