//! Student-facing question types.

/// A complete quiz question.
pub struct Question {
    /// The unique identifier for this question.
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
    pub fn new(id: QuestionId, prompt: String, points: u32, kind: QuestionKind) -> Self {
        Self {
            id,
            prompt,
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

/// Stable identifier for questions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuestionId(u32);

impl QuestionId {
    /// Creates a new question identifier.
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the raw numeric value of this identifier.
    pub fn value(self) -> u32 {
        self.0
    }
}

/// The type-specific data for a question.
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
}

/// A question with multiple choice options.
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
pub struct ChoiceOption {
    /// The unique identifier for this option.
    id: ChoiceId,
    /// The text shown for this option.
    text: String,
}

impl ChoiceOption {
    /// Creates a new choice option.
    pub fn new(id: ChoiceId, text: String) -> Self {
        Self { id, text }
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

/// A unique identifier for a choice option within a question.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChoiceId(u32);

impl ChoiceId {
    /// Creates a new choice identifier.
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the raw numeric value of this identifier.
    pub fn value(self) -> u32 {
        self.0
    }
}

/// A question that is answered in text.
pub struct TextAnswerQuestion {
    /// The maximum number of words allowed in the answer.
    word_limit: Option<usize>,
}

impl TextAnswerQuestion {
    /// Creates a new text answer question.
    pub fn new(word_limit: Option<usize>) -> Self {
        Self { word_limit }
    }

    /// Returns the maximum allowed word count, if one exists.
    pub fn word_limit(&self) -> Option<usize> {
        self.word_limit
    }
}

/// A question that is answered by matching items from two lists.
pub struct MatchingQuestion {
    /// The left list options.
    left: Vec<MatchItem>,
    /// The right list options.
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
pub struct MatchItem {
    /// The unique identifier for this item.
    id: MatchItemId,
    /// The text shown for this item.
    text: String,
}

impl MatchItem {
    /// Creates a new matching item.
    pub fn new(id: MatchItemId, text: String) -> Self {
        Self { id, text }
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

/// A unique identifier for a match item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MatchItemId(u32);

impl MatchItemId {
    /// Creates a new match item identifier.
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the raw numeric value of this identifier.
    pub fn value(self) -> u32 {
        self.0
    }
}

/// A question that is answered by ordering items.
pub struct OrderingQuestion {
    /// The items to be ordered.
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
pub struct OrderingItem {
    /// The unique identifier for this item.
    id: OrderingItemId,
    /// The text shown for this item.
    text: String,
}

impl OrderingItem {
    /// Creates a new ordering item.
    pub fn new(id: OrderingItemId, text: String) -> Self {
        Self { id, text }
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

/// A unique identifier for an ordering item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderingItemId(u32);

impl OrderingItemId {
    /// Creates a new ordering item identifier.
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the raw numeric value of this identifier.
    pub fn value(self) -> u32 {
        self.0
    }
}
