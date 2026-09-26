// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently, the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct `ReportCard` and the impl
// block to support alphabetical report cards in addition to numerical ones.

// TODO: Adjust the struct as described above.
// 使用泛型 T 来表示成绩类型
struct ReportCard<T> {
    grade: T,
    student_name: String,
    student_age: u8,
}

// TODO: Adjust the impl block as described above.
// 为所有实现了 Display trait 的类型实现 ReportCard
impl<T: std::fmt::Display> ReportCard<T> {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

fn main() {
    // You can optionally experiment here.
    // 数值成绩报告卡
    let numeric_report = ReportCard {
        grade: 3.8,
        student_name: "Alice".to_string(),
        student_age: 15,
    };
    
    // 字母成绩报告卡
    let alphabetic_report = ReportCard {
        grade: "A-",
        student_name: "Bob".to_string(),
        student_age: 16,
    };
    
    // 甚至可以使用整数成绩
    let integer_report = ReportCard {
        grade: 95,
        student_name: "Charlie".to_string(),
        student_age: 14,
    };
    
    println!("数值成绩: {}", numeric_report.print());
    println!("字母成绩: {}", alphabetic_report.print());
    println!("整数成绩: {}", integer_report.print());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}