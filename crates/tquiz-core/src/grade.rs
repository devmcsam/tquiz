//! Facilities for grading quizzes.

use core::error::Error;
use core::fmt;

use crate::answer::{AnswerKey, CorrectAnswer, StudentAnswer, Submission};
use crate::id::QuestionId;
use crate::question::QuestionKind;
use crate::quiz::QuizWithAnswerKey;

/// An error encountered while grading a submission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GradeError {
    /// The submission was made for a different quiz.
    QuizIdMismatch {
        expected: crate::id::QuizId,
        actual: crate::id::QuizId,
    },
    /// The submission contains an answer for a question not in the quiz.
    UnknownQuestion { question_id: QuestionId },
    /// The answer key does not contain an answer for a quiz question.
    MissingCorrectAnswer { question_id: QuestionId },
    /// The correct answer variant does not match the question type.
    CorrectAnswerTypeMismatch { question_id: QuestionId },
    /// The student's answer variant does not match the question type.
    StudentAnswerTypeMismatch { question_id: QuestionId },
}

impl fmt::Display for GradeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QuizIdMismatch { expected, actual } => {
                write!(
                    f,
                    "submission is for quiz {actual}, expected quiz {expected}"
                )
            }
            Self::UnknownQuestion { question_id } => {
                write!(f, "submission contains unknown question {question_id}")
            }
            Self::MissingCorrectAnswer { question_id } => {
                write!(f, "answer key has no answer for question {question_id}")
            }
            Self::CorrectAnswerTypeMismatch { question_id } => {
                write!(f, "answer key type does not match question {question_id}")
            }
            Self::StudentAnswerTypeMismatch { question_id } => {
                write!(
                    f,
                    "student answer type does not match question {question_id}"
                )
            }
        }
    }
}

impl Error for GradeError {}

// The grading result for one question.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestionGrade {
    question_id: QuestionId,
    earned_points: u64,
    possible_points: u64,
    correct: bool,
    pending_manual_grade: bool,
}

impl QuestionGrade {
    /// Returns the graded question's identifier.
    pub fn question_id(&self) -> QuestionId {
        self.question_id
    }

    /// Returns the points earned for this question.
    pub fn earned_points(&self) -> u64 {
        self.earned_points
    }

    /// Returns the maximum points available for this question.
    pub fn possible_points(&self) -> u64 {
        self.possible_points
    }

    /// Returns whether the answer was fully correct.
    ///
    /// Open-ended questions that are awaiting administrator grading return `false`
    /// until they are manually graded.
    pub fn is_correct(&self) -> bool {
        self.correct
    }

    /// Returns whether this question still needs administrator grading.
    pub fn is_pending_manual_grade(&self) -> bool {
        self.pending_manual_grade
    }
}

/// The result of grading a submission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GradeReport {
    quiz_id: crate::id::QuizId,
    /// Questions left to grade manually.
    left_to_grade: Vec<QuestionId>,
    question_grades: Vec<QuestionGrade>,
    earned_points: u64,
    possible_points: u64,
}

impl GradeReport {
    /// Returns the ID of the graded quiz.
    pub fn quiz_id(&self) -> crate::id::QuizId {
        self.quiz_id
    }

    /// Returns the questions left to grade manually.
    pub fn left_to_grade(&self) -> &[QuestionId] {
        &self.left_to_grade
    }

    /// Returns whether any questions still need manual grading.
    pub fn has_questions_left_to_grade(&self) -> bool {
        !self.left_to_grade.is_empty()
    }

    /// Returns the per-question results in quiz presentation order.
    pub fn question_grades(&self) -> &[QuestionGrade] {
        &self.question_grades
    }

    /// Returns the total points earned by automatically graded questions.
    ///
    /// This does not include points that may later be awarded for questions left
    /// for administrator grading.
    pub fn earned_points(&self) -> u64 {
        self.earned_points
    }

    /// Returns the total points available in the quiz, including manually graded
    /// open-ended questions.
    pub fn possible_points(&self) -> u64 {
        self.possible_points
    }
}

/// Grades a submission against the quiz and its answer key.
///
/// Unanswered automatically graded questions are marked incorrect. Open-ended
/// questions are left for administrator grading and do not require answer-key
/// entries. The function returns an error for unknown submitted question IDs,
/// missing key entries for automatically graded questions, or mismatched answer
/// types.
pub fn grade_quiz(
    quiz_with_key: &QuizWithAnswerKey,
    submission: &Submission,
) -> Result<GradeReport, GradeError> {
    let quiz = quiz_with_key.quiz();
    let answer_key: &AnswerKey = quiz_with_key.answer_key();

    if submission.quiz_id() != quiz.id() {
        return Err(GradeError::QuizIdMismatch {
            expected: quiz.id(),
            actual: submission.quiz_id(),
        });
    }

    for (question_id, _) in submission.iter() {
        if quiz.question(question_id).is_none() {
            return Err(GradeError::UnknownQuestion { question_id });
        }
    }

    let mut left_to_grade = Vec::new();
    let mut question_grades = Vec::with_capacity(quiz.len());
    let mut earned_points = 0_u64;
    let mut possible_points = 0_u64;

    for question in quiz.questions() {
        let question_id = question.id();
        let points = u64::from(question.points());

        possible_points += points;

        if matches!(question.kind(), QuestionKind::OpenEnded(_)) {
            match submission.get(question_id) {
                None | Some(StudentAnswer::OpenEndedAnswer(_)) => {
                    left_to_grade.push(question_id);

                    question_grades.push(QuestionGrade {
                        question_id,
                        earned_points: 0,
                        possible_points: points,
                        correct: false,
                        pending_manual_grade: true,
                    });

                    continue;
                }
                Some(_) => {
                    return Err(GradeError::StudentAnswerTypeMismatch { question_id });
                }
            }
        }

        let correct_answer = answer_key
            .get(question_id)
            .ok_or(GradeError::MissingCorrectAnswer { question_id })?;

        let student_answer = submission.get(question_id);
        let correct =
            is_answer_correct(question.kind(), correct_answer, student_answer, question_id)?;

        let points_earned = if correct { points } else { 0 };

        earned_points += points_earned;

        question_grades.push(QuestionGrade {
            question_id,
            earned_points: points_earned,
            possible_points: points,
            correct,
            pending_manual_grade: false,
        });
    }

    Ok(GradeReport {
        quiz_id: quiz.id(),
        left_to_grade,
        question_grades,
        earned_points,
        possible_points,
    })
}

fn is_answer_correct(
    question_kind: &QuestionKind,
    correct_answer: &CorrectAnswer,
    student_answer: Option<&StudentAnswer>,
    question_id: QuestionId,
) -> Result<bool, GradeError> {
    let correct = match (question_kind, correct_answer) {
        (QuestionKind::MultipleChoice(_), CorrectAnswer::MultipleChoice { choices }) => {
            match student_answer {
                None => false,
                Some(StudentAnswer::MultipleChoice { choices: selected }) => choices == selected,
                Some(_) => {
                    return Err(GradeError::StudentAnswerTypeMismatch { question_id });
                }
            }
        }
        (
            QuestionKind::TextAnswer(_),
            CorrectAnswer::TextAnswer {
                expected,
                case_sensitive,
            },
        ) => match student_answer {
            None => false,
            Some(StudentAnswer::TextAnswer(text)) => expected.iter().any(|answer| {
                if *case_sensitive {
                    text == answer
                } else {
                    text.eq_ignore_ascii_case(answer)
                }
            }),
            Some(_) => {
                return Err(GradeError::StudentAnswerTypeMismatch { question_id });
            }
        },
        (QuestionKind::TrueFalse, CorrectAnswer::TrueFalse(expected)) => match student_answer {
            None => false,
            Some(StudentAnswer::TrueFalse(actual)) => actual == expected,
            Some(_) => {
                return Err(GradeError::StudentAnswerTypeMismatch { question_id });
            }
        },
        (QuestionKind::Matching(_), CorrectAnswer::Matching { pairs }) => match student_answer {
            None => false,
            Some(StudentAnswer::Matching { pairs: actual }) => actual == pairs,
            Some(_) => {
                return Err(GradeError::StudentAnswerTypeMismatch { question_id });
            }
        },
        (QuestionKind::Ordering(_), CorrectAnswer::Ordering { order }) => match student_answer {
            None => false,
            Some(StudentAnswer::Ordering { order: actual }) => actual == order,
            Some(_) => {
                return Err(GradeError::StudentAnswerTypeMismatch { question_id });
            }
        },
        (QuestionKind::OpenEnded(_), _) => {
            return Err(GradeError::CorrectAnswerTypeMismatch { question_id });
        }
        _ => return Err(GradeError::CorrectAnswerTypeMismatch { question_id }),
    };

    Ok(correct)
}
