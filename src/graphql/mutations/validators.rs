use async_graphql::{CustomValidator, InputValueError};
use chrono::{Datelike, NaiveDate};

pub struct MonthBeginningValidator;

impl CustomValidator<NaiveDate> for MonthBeginningValidator {
    /// Check if `value` day of month is 1
    fn check(&self, value: &NaiveDate) -> Result<(), InputValueError<NaiveDate>> {
        if value.day() == 1 {
            Ok(())
        } else {
            Err(InputValueError::custom(format!(
                "Expected first day of the month, got `{}`",
                value
            )))
        }
    }
}
