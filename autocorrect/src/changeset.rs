use std::fmt;

use difference::{Changeset, Difference};

pub struct ChangesetWaring<'a> {
    pub changeset: &'a Changeset,
}

pub trait ChangesetOutput {
    fn as_warning(&self) -> ChangesetWaring;
}

impl ChangesetOutput for Changeset {
    fn as_warning(self: &difference::Changeset) -> ChangesetWaring {
        ChangesetWaring { changeset: self }
    }
}

impl fmt::Display for ChangesetWaring<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for d in &self.changeset.diffs {
            match *d {
                Difference::Same(ref x) => {
                    write!(f, "{}{}", x, self.changeset.split)?;
                }
                Difference::Add(ref x) => {
                    write!(f, "\x1b[92m{}\x1b[0m{}", x, self.changeset.split)?;
                }
                Difference::Rem(ref x) => {
                    write!(f, "\x1b[93m{}\x1b[0m{}", x, self.changeset.split)?;
                }
            }
        }
        Ok(())
    }
}
