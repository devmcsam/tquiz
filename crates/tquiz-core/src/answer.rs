//! Answer key and student submission types.

use crate::question::{ChoiceId, MatchItemId, OrderingItemId, QuestionId};

/// The administrator-owned answer key for a quiz.
pub struct AnswerKey {
    answers: Vec<CorrectAnswer>,
}

impl AnswerKey {
    /// Creates a new answer key.
    pub fn new(answers: Vec<CorrectAnswer>) -> Self {
        Self { answers }
    }

    /// Returns the correct answers in this answer key.
    pub fn answers(&self) -> &[CorrectAnswer] {
        &self.answers
    }
}

/// A correct answer for a single question.
pub enum CorrectAnswer {
    /// The correct choice or choices for a multiple choice question.
    MultipleChoice {
        question_id: QuestionId,
        choices: Vec<ChoiceId>,
    },
    /// The expected answer or answers for a text answer question.
    TextAnswer {
        question_id: QuestionId,
        expected_answers: Vec<String>,
        case_sensitive: bool,
    },
    /// The correct value for a true/false question.
    TrueFalse {
        question_id: QuestionId,
        correct: bool,
    },
    /// The correct pairs for a matching question.
    Matching {
        question_id: QuestionId,
        pairs: Vec<(MatchItemId, MatchItemId)>,
    },
    /// The correct order for an ordering question.
    Ordering {
        question_id: QuestionId,
        order: Vec<OrderingItemId>,
    },
}

impl CorrectAnswer {
    /// Creates a multiple choice correct answer.
    pub fn multiple_choice(question_id: QuestionId, choices: Vec<ChoiceId>) -> Self {
        Self::MultipleChoice {
            question_id,
            choices,
        }
    }

    /// Creates a text correct answer.
    pub fn text_answer(
        question_id: QuestionId,
        expected_answers: Vec<String>,
        case_sensitive: bool,
    ) -> Self {
        Self::TextAnswer {
            question_id,
            expected_answers,
            case_sensitive,
        }
    }

    /// Creates a true/false correct answer.
    pub fn true_false(question_id: QuestionId, correct: bool) -> Self {
        Self::TrueFalse {
            question_id,
            correct,
        }
    }

    /// Creates a matching correct answer.
    pub fn matching(question_id: QuestionId, pairs: Vec<(MatchItemId, MatchItemId)>) -> Self {
        Self::Matching { question_id, pairs }
    }

    /// Creates an ordering correct answer.
    pub fn ordering(question_id: QuestionId, order: Vec<OrderingItemId>) -> Self {
        Self::Ordering { question_id, order }
    }

    /// Returns the question identifier this answer belongs to.
    pub fn question_id(&self) -> QuestionId {
        match self {
            Self::MultipleChoice { question_id, .. }
            | Self::TextAnswer { question_id, .. }
            | Self::TrueFalse { question_id, .. }
            | Self::Matching { question_id, .. }
            | Self::Ordering { question_id, .. } => *question_id,
        }
    }
}

/// A student's submitted quiz answers.
pub struct Submission {
    answers: Vec<StudentAnswer>,
}

impl Submission {
    /// Creates a new student submission.
    pub fn new(answers: Vec<StudentAnswer>) -> Self {
        Self { answers }
    }

    /// Returns the submitted answers.
    pub fn answers(&self) -> &[StudentAnswer] {
        &self.answers
    }
}

/// A student's answer to a single question.
pub enum StudentAnswer {
    /// A student's selected multiple choice option or options.
    MultipleChoice {
        question_id: QuestionId,
        choices: Vec<ChoiceId>,
    },
    /// A student's text answer.
    TextAnswer {
        question_id: QuestionId,
        text: String,
    },
    /// A student's true/false answer.
    TrueFalse {
        question_id: QuestionId,
        value: bool,
    },
    /// A student's matching pairs.
    Matching {
        question_id: QuestionId,
        pairs: Vec<(MatchItemId, MatchItemId)>,
    },
    /// A student's ordering answer.
    Ordering {
        question_id: QuestionId,
        order: Vec<OrderingItemId>,
    },
}

impl StudentAnswer {
    /// Creates a multiple choice student answer.
    pub fn multiple_choice(question_id: QuestionId, choices: Vec<ChoiceId>) -> Self {
        Self::MultipleChoice {
            question_id,
            choices,
        }
    }

    /// Creates a text student answer.
    pub fn text_answer(question_id: QuestionId, text: String) -> Self {
        Self::TextAnswer { question_id, text }
    }

    /// Creates a true/false student answer.
    pub fn true_false(question_id: QuestionId, value: bool) -> Self {
        Self::TrueFalse { question_id, value }
    }

    /// Creates a matching student answer.
    pub fn matching(question_id: QuestionId, pairs: Vec<(MatchItemId, MatchItemId)>) -> Self {
        Self::Matching { question_id, pairs }
    }

    /// Creates an ordering student answer.
    pub fn ordering(question_id: QuestionId, order: Vec<OrderingItemId>) -> Self {
        Self::Ordering { question_id, order }
    }

    /// Returns the question identifier this answer belongs to.
    pub fn question_id(&self) -> QuestionId {
        match self {
            Self::MultipleChoice { question_id, .. }
            | Self::TextAnswer { question_id, .. }
            | Self::TrueFalse { question_id, .. }
            | Self::Matching { question_id, .. }
            | Self::Ordering { question_id, .. } => *question_id,
        }
    }
}
