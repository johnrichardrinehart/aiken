use std::fmt::Display;

use crate::{
    builtins::DefaultFunction,
    debruijn::{self, Converter},
};

/// This represents a program in Untyped Plutus Core.
/// A program contains a version tuple and a term.
/// It is generic because Term requires a generic type.
#[derive(Debug, Clone, PartialEq)]
pub struct Program<T> {
    pub version: (usize, usize, usize),
    pub term: Term<T>,
}

/// This represents a term in Untyped Plutus Core.
/// We need a generic type for the different forms that a program may be in.
/// Specifically, `Var` and `parameter_name` in `Lambda` can be a `Name`,
/// `NamedDebruijn`, or `DeBruijn`. When encoded to flat for on chain usage
/// we must encode using the `DeBruijn` form.
#[derive(Debug, Clone, PartialEq)]
pub enum Term<T> {
    // tag: 0
    Var(T),
    // tag: 1
    Delay(Box<Term<T>>),
    // tag: 2
    Lambda {
        parameter_name: T,
        body: Box<Term<T>>,
    },
    // tag: 3
    Apply {
        function: Box<Term<T>>,
        argument: Box<Term<T>>,
    },
    // tag: 4
    Constant(Constant),
    // tag: 5
    Force(Box<Term<T>>),
    // tag: 6
    Error,
    // tag: 7
    Builtin(DefaultFunction),
}

/// A container for the various constants that are available
/// in Untyped Plutus Core. Used in the `Constant` variant of `Term`.
#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    // tag: 0
    Integer(isize),
    // tag: 1
    ByteString(Vec<u8>),
    // tag: 2
    String(String),
    // tag: 3
    Char(char),
    // tag: 4
    Unit,
    // tag: 5
    Bool(bool),
}

/// A Name containing it's parsed textual representation
/// and a unique id from string interning. The Name's text is
/// interned during parsing.
#[derive(Debug, Clone, PartialEq)]
pub struct Name {
    pub text: String,
    pub unique: Unique,
}

/// A unique id used for string interning.
#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
pub struct Unique(isize);

impl Unique {
    /// Create a new unique id.
    pub fn new(unique: isize) -> Self {
        Unique(unique)
    }

    /// Increment the available unique id. This is used during
    /// string interning to get the next available unique id.
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

impl From<isize> for Unique {
    fn from(i: isize) -> Self {
        Unique(i)
    }
}

impl From<Unique> for isize {
    fn from(d: Unique) -> Self {
        d.0
    }
}

impl Display for Unique {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Similar to `Name` but for Debruijn indices.
/// `Name` is replaced by `NamedDebruijn` when converting
/// program to it's debruijn form.
#[derive(Debug, Clone, PartialEq)]
pub struct NamedDeBruijn {
    pub text: String,
    pub index: DeBruijn,
}

/// This is useful for decoding a on chain program into debruijn form.
/// It allows for injecting fake textual names while also using Debruijn for decoding
/// without having to loop through twice.
#[derive(Debug, Clone, PartialEq)]
pub struct FakeNamedDeBruijn(NamedDeBruijn);

impl From<DeBruijn> for FakeNamedDeBruijn {
    fn from(d: DeBruijn) -> Self {
        FakeNamedDeBruijn(d.into())
    }
}

impl From<FakeNamedDeBruijn> for DeBruijn {
    fn from(d: FakeNamedDeBruijn) -> Self {
        d.0.into()
    }
}

impl From<FakeNamedDeBruijn> for NamedDeBruijn {
    fn from(d: FakeNamedDeBruijn) -> Self {
        d.0
    }
}

impl From<NamedDeBruijn> for FakeNamedDeBruijn {
    fn from(d: NamedDeBruijn) -> Self {
        FakeNamedDeBruijn(d)
    }
}

/// Represents a debruijn index.
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct DeBruijn(usize);

impl DeBruijn {
    /// Create a new debruijn index.
    pub fn new(index: usize) -> Self {
        DeBruijn(index)
    }
}

impl From<usize> for DeBruijn {
    fn from(i: usize) -> Self {
        DeBruijn(i)
    }
}

impl From<DeBruijn> for usize {
    fn from(d: DeBruijn) -> Self {
        d.0
    }
}

impl From<NamedDeBruijn> for DeBruijn {
    fn from(n: NamedDeBruijn) -> Self {
        n.index
    }
}

impl From<DeBruijn> for NamedDeBruijn {
    fn from(index: DeBruijn) -> Self {
        NamedDeBruijn {
            // Inject fake name. We got `i` from the Plutus code base.
            text: String::from("i"),
            index,
        }
    }
}

/// Convert a Parsed `Program` to a `Program` in `NamedDebruijn` form.
/// This checks for any Free Uniques in the `Program` and returns an error if found.
impl TryFrom<Program<Name>> for Program<NamedDeBruijn> {
    type Error = debruijn::Error;

    fn try_from(value: Program<Name>) -> Result<Self, Self::Error> {
        Ok(Program::<NamedDeBruijn> {
            version: value.version,
            term: value.term.try_into()?,
        })
    }
}

/// Convert a Parsed `Term` to a `Term` in `NamedDebruijn` form.
/// This checks for any Free Uniques in the `Term` and returns an error if found.
impl TryFrom<Term<Name>> for Term<NamedDeBruijn> {
    type Error = debruijn::Error;

    fn try_from(value: Term<Name>) -> Result<Self, debruijn::Error> {
        let mut converter = Converter::new();

        let term = converter.name_to_named_debruijn(value)?;

        Ok(term)
    }
}

/// Convert a Parsed `Program` to a `Program` in `Debruijn` form.
/// This checks for any Free Uniques in the `Program` and returns an error if found.
impl TryFrom<Program<Name>> for Program<DeBruijn> {
    type Error = debruijn::Error;

    fn try_from(value: Program<Name>) -> Result<Self, Self::Error> {
        Ok(Program::<DeBruijn> {
            version: value.version,
            term: value.term.try_into()?,
        })
    }
}

/// Convert a Parsed `Term` to a `Term` in `Debruijn` form.
/// This checks for any Free Uniques in the `Program` and returns an error if found.
impl TryFrom<Term<Name>> for Term<DeBruijn> {
    type Error = debruijn::Error;

    fn try_from(value: Term<Name>) -> Result<Self, debruijn::Error> {
        let mut converter = Converter::new();

        let term = converter.name_to_debruijn(value)?;

        Ok(term)
    }
}

impl TryFrom<Program<NamedDeBruijn>> for Program<Name> {
    type Error = debruijn::Error;

    fn try_from(value: Program<NamedDeBruijn>) -> Result<Self, Self::Error> {
        Ok(Program::<Name> {
            version: value.version,
            term: value.term.try_into()?,
        })
    }
}

impl TryFrom<Term<NamedDeBruijn>> for Term<Name> {
    type Error = debruijn::Error;

    fn try_from(value: Term<NamedDeBruijn>) -> Result<Self, debruijn::Error> {
        let mut converter = Converter::new();

        let term = converter.named_debruijn_to_name(value)?;

        Ok(term)
    }
}

impl From<Program<NamedDeBruijn>> for Program<DeBruijn> {
    fn from(value: Program<NamedDeBruijn>) -> Self {
        Program::<DeBruijn> {
            version: value.version,
            term: value.term.into(),
        }
    }
}

impl From<Term<NamedDeBruijn>> for Term<DeBruijn> {
    fn from(value: Term<NamedDeBruijn>) -> Self {
        let mut converter = Converter::new();

        converter.named_debruijn_to_debruijn(value)
    }
}

impl From<Program<NamedDeBruijn>> for Program<FakeNamedDeBruijn> {
    fn from(value: Program<NamedDeBruijn>) -> Self {
        Program::<FakeNamedDeBruijn> {
            version: value.version,
            term: value.term.into(),
        }
    }
}

impl From<Term<NamedDeBruijn>> for Term<FakeNamedDeBruijn> {
    fn from(value: Term<NamedDeBruijn>) -> Self {
        let mut converter = Converter::new();

        converter.named_debruijn_to_fake_named_debruijn(value)
    }
}

impl TryFrom<Program<DeBruijn>> for Program<Name> {
    type Error = debruijn::Error;

    fn try_from(value: Program<DeBruijn>) -> Result<Self, Self::Error> {
        Ok(Program::<Name> {
            version: value.version,
            term: value.term.try_into()?,
        })
    }
}

impl TryFrom<Term<DeBruijn>> for Term<Name> {
    type Error = debruijn::Error;

    fn try_from(value: Term<DeBruijn>) -> Result<Self, debruijn::Error> {
        let mut converter = Converter::new();

        let term = converter.debruijn_to_name(value)?;

        Ok(term)
    }
}

impl From<Program<DeBruijn>> for Program<NamedDeBruijn> {
    fn from(value: Program<DeBruijn>) -> Self {
        Program::<NamedDeBruijn> {
            version: value.version,
            term: value.term.into(),
        }
    }
}

impl From<Term<DeBruijn>> for Term<NamedDeBruijn> {
    fn from(value: Term<DeBruijn>) -> Self {
        let mut converter = Converter::new();

        converter.debruijn_to_named_debruijn(value)
    }
}

impl From<Program<FakeNamedDeBruijn>> for Program<NamedDeBruijn> {
    fn from(value: Program<FakeNamedDeBruijn>) -> Self {
        Program::<NamedDeBruijn> {
            version: value.version,
            term: value.term.into(),
        }
    }
}

impl From<Term<FakeNamedDeBruijn>> for Term<NamedDeBruijn> {
    fn from(value: Term<FakeNamedDeBruijn>) -> Self {
        let mut converter = Converter::new();

        converter.fake_named_debruijn_to_named_debruijn(value)
    }
}
