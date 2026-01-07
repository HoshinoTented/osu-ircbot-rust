use std::fmt::Display;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Default)]
pub struct IrcName(String);

impl IrcName {
  pub fn eq<S : AsRef<str>>(&self, other: &S) -> bool {
    other.as_ref().replace(" ", "_") == self.0
  }

  pub fn eq_raw(&self, other: &'static str) -> bool {
    self.0 == other
  }

  pub fn new<S : AsRef<str>>(from: S) -> IrcName {
    IrcName(from.as_ref().replace(" ", "_"))
  }

  pub fn inner(&self) -> &String {
    &self.0
  }
}

impl Display for IrcName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<IrcName> for String {
    fn into(self) -> IrcName {
        IrcName::new(self)
    }
}

impl Into<IrcName> for &String {
    fn into(self) -> IrcName {
        IrcName::new(self)
    }
}

impl Into<IrcName> for &str {
    fn into(self) -> IrcName {
        IrcName::new(self)
    }
}