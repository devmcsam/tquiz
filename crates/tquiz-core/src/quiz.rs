//! Quiz container types.

use crate::answer::AnswerKey;
use crate::question::Question;

/// A student-facing quiz.
///
/// This type should be safe to send to test takers because it does not contain
/// the answer key.
pub struct Quiz {
    title: String,
    description: Option<String>,
    questions: Vec<Question>,
}

impl Quiz {
    /// Creates a new quiz.
    pub fn new(title: String, description: Option<String>, questions: Vec<Question>) -> Self {
        Self {
            title,
            description,
            questions,
        }
    }

    /// Returns the quiz title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the quiz description, if one exists.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the questions in this quiz.
    pub fn questions(&self) -> &[Question] {
        &self.questions
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
/// Do not send this whole type to test takers.
pub struct QuizWithAnswerKey {
    quiz: Quiz,
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
