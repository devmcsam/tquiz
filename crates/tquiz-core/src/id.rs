//! Strongly typed identifiers.
//!
//! Each identifier is a `Copy` newtype so that it can be passed around freely and used
//! as a map key without allocating.

use core::fmt;

macro_rules! define_id {
    ($(#[$meta:meta])* $name:ident($inner:ty)) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "serde", serde(transparent))]
        pub struct $name($inner);

        impl $name {
            /// Creates a new identifier from its raw value.
            pub const fn new(value: $inner) -> Self {
                Self(value)
            }

            /// Returns the raw value of this identifier.
            pub const fn value(self) -> $inner {
                self.0
            }
        }

        impl From<$inner> for $name {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $inner {
            fn from(id: $name) -> Self {
                id.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.0, f)
            }
        }
    };
}

define_id! {
    /// Identifies a quiz.
    ///
    /// Must be unique across every quiz an administrator distributes, because submissions
    /// reference quizzes by this identifier.
    QuizId(u64)
}

define_id! {
    /// Identifies a question within a quiz.
    QuestionId(u32)
}

define_id! {
    /// Identifies a choice option within a multiple choice question.
    ChoiceId(u32)
}

define_id! {
    /// Identifies an item within a matching question.
    MatchItemId(u32)
}

define_id! {
    /// Identifies an item within an ordering question.
    OrderingItemId(u32)
}
