//! Quiz container types.

use crate::answer::AnswerKey;
use crate::id::{QuestionId, QuizId};
use crate::question::Question;

/// A student-facing quiz.
///
/// This type should be safe to send to test takers because it does not contain
/// the answer key.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Quiz {
    /// The unique identifier for this quiz. Submissions reference this.
    id: QuizId,
    /// The title or name of the quiz.
    title: String,
    /// The description of the quiz, if one exists.
    description: Option<String>,
    /// The questions within this quiz, in presentation order.
    questions: Vec<Question>,
}

impl Quiz {
    /// Creates a new quiz.
    pub fn new(
        id: QuizId,
        title: impl Into<String>,
        description: Option<String>,
        questions: Vec<Question>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            description,
            questions,
        }
    }

    /// Returns this quiz's identifier.
    pub fn id(&self) -> QuizId {
        self.id
    }

    /// Returns the quiz title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the quiz description, if one exists.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the questions in this quiz, in presentation order.
    pub fn questions(&self) -> &[Question] {
        &self.questions
    }

    /// Returns the question with the given identifier, if it exists.
    pub fn question(&self, id: QuestionId) -> Option<&Question> {
        self.questions.iter().find(|q| q.id() == id)
    }

    /// Returns the total number of points available in this quiz.
    pub fn total_points(&self) -> u32 {
        self.questions.iter().map(Question::points).sum()
    }

    /// Returns true if this quiz has no questions.
    pub fn is_empty(&self) -> bool {
        self.questions.is_empty()
    }

    /// Returns the number of questions in this quiz.
    pub fn len(&self) -> usize {
        self.questions.len()
    }
}

/// An administrator-owned quiz package.
///
/// This contains both the student-facing quiz and the private answer key.
/// Do not send this whole type to test takers; use [`QuizWithAnswerKey::quiz`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QuizWithAnswerKey {
    /// The student-facing quiz.
    quiz: Quiz,
    /// The correct answers for the given quiz.
    answer_key: AnswerKey,
}

impl QuizWithAnswerKey {
    /// Creates a new quiz package containing a quiz and its answer key.
    pub fn new(quiz: Quiz, answer_key: AnswerKey) -> Self {
        Self { quiz, answer_key }
    }

    /// Returns the student-facing quiz.
    pub fn quiz(&self) -> &Quiz {
        &self.quiz
    }

    /// Returns the administrator-only answer key.
    pub fn answer_key(&self) -> &AnswerKey {
        &self.answer_key
    }

    /// Splits this package into its quiz and answer key.
    pub fn into_parts(self) -> (Quiz, AnswerKey) {
        (self.quiz, self.answer_key)
    }
}
