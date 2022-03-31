// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

mod record;
mod type_;

use crate::{
    templates::{record::Record, type_::Type},
    Annotation,
    Identifier,
};
use snarkvm_circuits_types::prelude::*;

use core::fmt;

/// A template is a custom or record type that represents a collection of circuits.
/// A template does not have a mode; rather its individual members are annotated with modes.
/// A template is defined by an identifier (such as `message`) and a list of members,
/// such as `[(sender, address.public), (amount, i64.private)]`, where the left entry is an identifier,
/// and the right entry is a type annotation.
///
/// A register member format is used to access individual members of a template. For example,
/// if the `record` template is assigned to register `r0`, individual members can be accessed
/// as `r0.owner` or `r0.value`. This generalizes to the format, i.e. `r{locator}.{member}`.
#[derive(Clone, Debug)]
pub enum Template<E: Environment> {
    /// A type consists of its identifier and a list of members.
    Type(Type<E>),
    /// A record consists of its identifier and a list of members.
    Record(Record<E>),
}

impl<E: Environment> Template<E> {
    /// Returns the name of the template.
    #[inline]
    pub fn name(&self) -> &Identifier<E> {
        match self {
            Self::Type(type_) => type_.name(),
            Self::Record(record) => record.name(),
        }
    }

    /// Returns the members of the template.
    #[inline]
    pub fn members(&self) -> &[(Identifier<E>, Annotation<E>)] {
        match self {
            Self::Type(type_) => type_.members(),
            Self::Record(record) => record.members(),
        }
    }
}

impl<E: Environment> Parser for Template<E> {
    type Environment = E;

    /// Parses a string into a template.
    #[inline]
    fn parse(string: &str) -> ParserResult<Self> {
        alt((map(Type::parse, |type_| Self::Type(type_)), map(Record::parse, |record| Self::Record(record))))(string)
    }
}

impl<E: Environment> fmt::Display for Template<E> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Type(type_) => type_.fmt(f),
            Self::Record(record) => record.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_circuits_types::environment::Circuit;

    type E = Circuit;

    #[test]
    fn test_template_parse() {
        let message = Template::<E>::parse(
            r"
type message:
    sender as address.public;
    amount as i64.private;
",
        )
        .unwrap()
        .1;
        assert_eq!(message.name(), &Identifier::from_str("message"));
        assert_eq!(message.members().len(), 2);
        assert_eq!(message.members()[0].0, Identifier::from_str("sender"));
        assert_eq!(message.members()[0].1, Annotation::from_str("address.public"));
        assert_eq!(message.members()[1].0, Identifier::from_str("amount"));
        assert_eq!(message.members()[1].1, Annotation::from_str("i64.private"));

        let token = Template::<E>::parse(
            r"
record token:
    owner as address.public;
    amount as i64.private;
",
        )
        .unwrap()
        .1;
        assert_eq!(token.name(), &Identifier::from_str("token"));
        assert_eq!(token.members().len(), 2);
        assert_eq!(token.members()[0].0, Identifier::from_str("owner"));
        assert_eq!(token.members()[0].1, Annotation::from_str("address.public"));
        assert_eq!(token.members()[1].0, Identifier::from_str("amount"));
        assert_eq!(token.members()[1].1, Annotation::from_str("i64.private"));
    }

    #[test]
    fn test_template_display() {
        let expected = "type message:\n    sender as address.public;\n    amount as i64.private;";
        let message = Template::<E>::parse(expected).unwrap().1;
        assert_eq!(expected, format!("{}", message));

        let expected = "record token:\n    owner as address.public;\n    amount as i64.private;";
        let token = Template::<E>::parse(expected).unwrap().1;
        assert_eq!(expected, format!("{}", token));
    }
}
