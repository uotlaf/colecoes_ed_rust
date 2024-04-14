use std::str::FromStr;

/// Enum for main menu match
pub enum MainOption {
    Create,
    Insert,
    Show,
    Search,
    Remove,
    Clean,
    Destroy,
    Exit
}

impl FromStr for MainOption {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().unwrap() {
            '1' => Ok(MainOption::Create),
            '2' => Ok(MainOption::Insert),
            '3' => Ok(MainOption::Show),
            '4' => Ok(MainOption::Search),
            '5' => Ok(MainOption::Remove),
            '6' => Ok(MainOption::Clean),
            '7' => Ok(MainOption::Destroy),
            '8' => Ok(MainOption::Exit),
            _ => Err(())
        }
    }
}
