//! Student-facing question types.

use crate::id::{ChoiceId, MatchItemId, OrderingItemId, QuestionId};

/// A complete quiz question.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Question {
    /// The unique identifier for this question within its quiz.
    id: QuestionId,
    /// The prompt given with this question.
    prompt: String,
    /// The amount of points this question is worth.
    points: u32,
    /// What kind of question this is.
    kind: QuestionKind,
}

impl Question {
    /// Creates a new question.
    pub fn new(id: QuestionId, prompt: impl Into<String>, points: u32, kind: QuestionKind) -> Self {
        Self {
            id,
            prompt: prompt.into(),
            points,
            kind,
        }
    }

    /// Returns this question's identifier.
    pub fn id(&self) -> QuestionId {
        self.id
    }

    /// Returns this question's prompt.
    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    /// Returns how many points this question is worth.
    pub fn points(&self) -> u32 {
        self.points
    }

    /// Returns this question's type-specific data.
    pub fn kind(&self) -> &QuestionKind {
        &self.kind
    }
}

/// The type-specific data for a question.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum QuestionKind {
    /// A question where one or more options are selected.
    MultipleChoice(MultipleChoiceQuestion),
    /// A question where the answer is given in text.
    TextAnswer(TextAnswerQuestion),
    /// A question where the only possible answers are true or false.
    TrueFalse,
    /// A question where items from two lists are matched.
    Matching(MatchingQuestion),
    /// A question where items are ordered in a specific order.
    Ordering(OrderingQuestion),
    /// A question where the answer is given in text but must be graded by the administrator.
    OpenEnded(OpenEndedQuestion),
}

/// A question with multiple choice options.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MultipleChoiceQuestion {
    /// The options available on this question.
    options: Vec<ChoiceOption>,
    /// Whether this question allows selecting multiple choices.
    allow_multiple: bool,
}

impl MultipleChoiceQuestion {
    /// Creates a new multiple choice question.
    pub fn new(options: Vec<ChoiceOption>, allow_multiple: bool) -> Self {
        Self {
            options,
            allow_multiple,
        }
    }

    /// Returns the options available for this question.
    pub fn options(&self) -> &[ChoiceOption] {
        &self.options
    }

    /// Returns whether multiple options may be selected.
    pub fn allows_multiple(&self) -> bool {
        self.allow_multiple
    }
}

/// A single option in a multiple choice question.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChoiceOption {
    /// The unique identifier for this option within its question.
    id: ChoiceId,
    /// The text shown for this option.
    text: String,
}

impl ChoiceOption {
    /// Creates a new choice option.
    pub fn new(id: ChoiceId, text: impl Into<String>) -> Self {
        Self {
            id,
            text: text.into(),
        }
    }

    /// Returns this option's identifier.
    pub fn id(&self) -> ChoiceId {
        self.id
    }

    /// Returns this option's text.
    pub fn text(&self) -> &str {
        &self.text
    }
}

/// A question that is answered in text.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextAnswerQuestion {
    /// The maximum number of words allowed in the answer.
    word_limit: Option<u32>,
}

impl TextAnswerQuestion {
    /// Creates a new text answer question.
    pub fn new(word_limit: Option<u32>) -> Self {
        Self { word_limit }
    }

    /// Returns the maximum allowed word count, if one exists.
    pub fn word_limit(&self) -> Option<u32> {
        self.word_limit
    }
}

/// A question that is answered by matching items from two lists.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MatchingQuestion {
    /// The left list options.
    left: Vec<MatchItem>,
    /// The right list options. May contain distractors with no matching left item.
    right: Vec<MatchItem>,
}

impl MatchingQuestion {
    /// Creates a new matching question.
    pub fn new(left: Vec<MatchItem>, right: Vec<MatchItem>) -> Self {
        Self { left, right }
    }

    /// Returns the left-side matching items.
    pub fn left(&self) -> &[MatchItem] {
        &self.left
    }

    /// Returns the right-side matching items.
    pub fn right(&self) -> &[MatchItem] {
        &self.right
    }
}

/// An item within a matching question.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MatchItem {
    /// The unique identifier for this item within its question (across both lists).
    id: MatchItemId,
    /// The text shown for this item.
    text: String,
}

impl MatchItem {
    /// Creates a new matching item.
    pub fn new(id: MatchItemId, text: impl Into<String>) -> Self {
        Self {
            id,
            text: text.into(),
        }
    }

    /// Returns this item's identifier.
    pub fn id(&self) -> MatchItemId {
        self.id
    }

    /// Returns this item's text.
    pub fn text(&self) -> &str {
        &self.text
    }
}

/// A question that is answered by ordering items.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderingQuestion {
    /// The items to be ordered, in the (shuffled) order they are shown to the student.
    items: Vec<OrderingItem>,
}

impl OrderingQuestion {
    /// Creates a new ordering question.
    pub fn new(items: Vec<OrderingItem>) -> Self {
        Self { items }
    }

    /// Returns the items to be ordered.
    pub fn items(&self) -> &[OrderingItem] {
        &self.items
    }
}

/// An item within an ordering question.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrderingItem {
    /// The unique identifier for this item within its question.
    id: OrderingItemId,
    /// The text shown for this item.
    text: String,
}

impl OrderingItem {
    /// Creates a new ordering item.
    pub fn new(id: OrderingItemId, text: impl Into<String>) -> Self {
        Self {
            id,
            text: text.into(),
        }
    }

    /// Returns this item's identifier.
    pub fn id(&self) -> OrderingItemId {
        self.id
    }

    /// Returns this item's text.
    pub fn text(&self) -> &str {
        &self.text
    }
}

/// A question answered in text and must be graded by the administrator.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenEndedQuestion {
    word_limit: Option<u32>,
}

impl OpenEndedQuestion {
    pub fn new(limit: Option<u32>) -> Self {
        Self { word_limit: limit }
    }

    pub fn word_limit(&self) -> Option<u32> {
        self.word_limit
    }
}
