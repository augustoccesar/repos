use std::env;

pub enum Shell {
    Zsh,
    Fish,
    Other,
}

impl Shell {
    pub fn from_env() -> Self {
        let shell = env!("SHELL").split("/").last();

        match shell {
            Some(shell) => match shell {
                "zsh" => Self::Zsh,
                "fish" => Self::Fish,
                _ => Self::Other,
            },
            None => Self::Other,
        }
    }
}
