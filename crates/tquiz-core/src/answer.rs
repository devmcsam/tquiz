//! Answer key and student submission types.
//!
//! Both [`AnswerKey`] and [`Submission`] store answers in a map keyed by
//! [`QuestionId`]. This gives O(log n) lookups when grading, makes duplicate
//! answers for a question unrepresentable, and keeps serialized output
//! deterministic.

use std::collections::{BTreeMap, BTreeSet};

use crate::id::{ChoiceId, MatchItemId, OrderingItemId, QuestionId, QuizId};

/// The administrator-owned answer key for a quiz.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnswerKey {
    /// The correct answer for each question.
    answers: BTreeMap<QuestionId, CorrectAnswer>,
}

impl AnswerKey {
    /// Creates an empty answer key.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts the correct answer for a question.
    ///
    /// Returns the previous answer for that question, if one existed.
    pub fn insert(
        &mut self,
        question_id: QuestionId,
        answer: CorrectAnswer,
    ) -> Option<CorrectAnswer> {
        self.answers.insert(question_id, answer)
    }

    /// Returns the correct answer for a question, if one exists.
    pub fn get(&self, question_id: QuestionId) -> Option<&CorrectAnswer> {
        self.answers.get(&question_id)
    }

    /// Iterates over every `(question, correct answer)` pair in question-id order.
    pub fn iter(&self) -> impl Iterator<Item = (QuestionId, &CorrectAnswer)> {
        self.answers.iter().map(|(id, answer)| (*id, answer))
    }

    /// Returns the number of answers in this key.
    pub fn len(&self) -> usize {
        self.answers.len()
    }

    /// Returns true if this key contains no answers.
    pub fn is_empty(&self) -> bool {
        self.answers.is_empty()
    }
}

impl FromIterator<(QuestionId, CorrectAnswer)> for AnswerKey {
    fn from_iter<I: IntoIterator<Item = (QuestionId, CorrectAnswer)>>(iter: I) -> Self {
        Self {
            answers: iter.into_iter().collect(),
        }
    }
}

/// The correct answer for a single question.
///
/// The variant must correspond to the [`QuestionKind`](crate::question::QuestionKind)
/// of the question it is keyed under; a grader should treat a mismatch as an error.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CorrectAnswer {
    /// The correct choice or choices for a multiple choice question.
    MultipleChoice {
        /// The set of correct choices.
        choices: BTreeSet<ChoiceId>,
    },
    /// The accepted answers for a text answer question.
    TextAnswer {
        /// Any of these answers is accepted as correct.
        expected: Vec<String>,
        /// Whether comparison against `expected` is case sensitive.
        case_sensitive: bool,
    },
    /// The correct value for a true/false question.
    TrueFalse(bool),
    /// The correct pairs for a matching question.
    Matching {
        /// Maps each left-side item to its correct right-side item.
        pairs: BTreeMap<MatchItemId, MatchItemId>,
    },
    /// The correct order for an ordering question.
    Ordering {
        /// The item identifiers in their correct order.
        order: Vec<OrderingItemId>,
    },
}

impl CorrectAnswer {
    /// Creates a multiple choice correct answer.
    pub fn multiple_choice(choices: impl IntoIterator<Item = ChoiceId>) -> Self {
        Self::MultipleChoice {
            choices: choices.into_iter().collect(),
        }
    }

    /// Creates a text correct answer.
    pub fn text_answer(
        expected: impl IntoIterator<Item = impl Into<String>>,
        case_sensitive: bool,
    ) -> Self {
        Self::TextAnswer {
            expected: expected.into_iter().map(Into::into).collect(),
            case_sensitive,
        }
    }

    /// Creates a true/false correct answer.
    pub fn true_false(correct: bool) -> Self {
        Self::TrueFalse(correct)
    }

    /// Creates a matching correct answer from `(left, right)` pairs.
    pub fn matching(pairs: impl IntoIterator<Item = (MatchItemId, MatchItemId)>) -> Self {
        Self::Matching {
            pairs: pairs.into_iter().collect(),
        }
    }

    /// Creates an ordering correct answer.
    pub fn ordering(order: impl IntoIterator<Item = OrderingItemId>) -> Self {
        Self::Ordering {
            order: order.into_iter().collect(),
        }
    }
}

/// A student's submitted answers for a single quiz.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Submission {
    /// The quiz this submission answers.
    quiz_id: QuizId,
    /// The student's answer for each question they answered.
    answers: BTreeMap<QuestionId, StudentAnswer>,
}

impl Submission {
    /// Creates an empty submission for the given quiz.
    pub fn new(quiz_id: QuizId) -> Self {
        Self {
            quiz_id,
            answers: BTreeMap::new(),
        }
    }

    /// Returns the identifier of the quiz this submission answers.
    pub fn quiz_id(&self) -> QuizId {
        self.quiz_id
    }

    /// Records the student's answer to a question.
    ///
    /// Returns the previous answer for that question, if one existed.
    pub fn insert(
        &mut self,
        question_id: QuestionId,
        answer: StudentAnswer,
    ) -> Option<StudentAnswer> {
        self.answers.insert(question_id, answer)
    }

    /// Returns the student's answer to a question, if they answered it.
    pub fn get(&self, question_id: QuestionId) -> Option<&StudentAnswer> {
        self.answers.get(&question_id)
    }

    /// Iterates over every `(question, student answer)` pair in question-id order.
    pub fn iter(&self) -> impl Iterator<Item = (QuestionId, &StudentAnswer)> {
        self.answers.iter().map(|(id, answer)| (*id, answer))
    }

    /// Returns the number of answered questions.
    pub fn len(&self) -> usize {
        self.answers.len()
    }

    /// Returns true if no questions were answered.
    pub fn is_empty(&self) -> bool {
        self.answers.is_empty()
    }
}

/// A student's answer to a single question.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StudentAnswer {
    /// The selected multiple choice option or options.
    MultipleChoice {
        /// The set of selected choices.
        choices: BTreeSet<ChoiceId>,
    },
    /// A free-form text answer.
    TextAnswer(String),
    /// A true/false answer.
    TrueFalse(bool),
    /// The student's matching pairs.
    Matching {
        /// Maps each left-side item to the right-side item the student chose.
        pairs: BTreeMap<MatchItemId, MatchItemId>,
    },
    /// The student's ordering.
    Ordering {
        /// The item identifiers in the order the student chose.
        order: Vec<OrderingItemId>,
    },
    /// A text answer to an open ended question.
    OpenEndedAnswer(String),
}

impl StudentAnswer {
    /// Creates a multiple choice student answer.
    pub fn multiple_choice(choices: impl IntoIterator<Item = ChoiceId>) -> Self {
        Self::MultipleChoice {
            choices: choices.into_iter().collect(),
        }
    }

    /// Creates a text student answer.
    pub fn text_answer(text: impl Into<String>) -> Self {
        Self::TextAnswer(text.into())
    }

    /// Creates a true/false student answer.
    pub fn true_false(value: bool) -> Self {
        Self::TrueFalse(value)
    }

    /// Creates a matching student answer from `(left, right)` pairs.
    pub fn matching(pairs: impl IntoIterator<Item = (MatchItemId, MatchItemId)>) -> Self {
        Self::Matching {
            pairs: pairs.into_iter().collect(),
        }
    }

    /// Creates an ordering student answer.
    pub fn ordering(order: impl IntoIterator<Item = OrderingItemId>) -> Self {
        Self::Ordering {
            order: order.into_iter().collect(),
        }
    }

    pub fn open_ended_answer(text: impl Into<String>) -> Self {
        Self::OpenEndedAnswer(text.into())
    }
}
