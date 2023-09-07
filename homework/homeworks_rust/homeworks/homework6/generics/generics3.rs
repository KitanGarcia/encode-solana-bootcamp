// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5).
// However, the school also issues alphabetical grades (A+ -> F-) and needs
// to be able to print both types of report card!

// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to "A+"
// to show that your changes allow alphabetical grades.

// Execute 'rustlings hint generics3' for hints!

use std::fmt;
use std::fmt::Display;


// Define enum to for grade to be either type
enum Grade {
    Number(f32),
    Letter(String),
}
pub struct ReportCard {
    grade: Grade, // set to type Grade enum allowing for either type
    pub student_name: String,
    pub student_age: u8,
}

// Implement how Grade is displayed.
// Define how Grade should be printed
impl Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match &*self {
           Grade::Number(number) => write!(f, "{}", number),
           Grade::Letter(letter) => write!(f, "{}", letter),
       }
    }
}

impl ReportCard {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: Grade::Number(2.1),
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: Grade::Letter("A+".to_string()),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
