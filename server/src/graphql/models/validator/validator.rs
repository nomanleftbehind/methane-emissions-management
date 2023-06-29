use super::super::input::MonthRangeInput;
use async_graphql::{CustomValidator, InputValueError};
use chrono::{Datelike, NaiveDate};

pub struct MonthBeginningValidator;

impl CustomValidator<NaiveDate> for MonthBeginningValidator {
    fn check(&self, value: &NaiveDate) -> Result<(), InputValueError<NaiveDate>> {
        if value.day() == 1 {
            Ok(())
        } else {
            Err(InputValueError::custom(format!(
                "Expected first day of the month, got `{}`.",
                value
            )))
        }
    }
}

pub struct MonthRangeValidator {
    allowed_month_range: u32,
}

impl MonthRangeValidator {
    pub fn new(n: u32) -> Self {
        Self {
            allowed_month_range: n,
        }
    }
}

impl CustomValidator<MonthRangeInput> for MonthRangeValidator {
    fn check(
        &self,
        MonthRangeInput {
            from_month,
            to_month,
        }: &MonthRangeInput,
    ) -> Result<(), InputValueError<MonthRangeInput>> {
        let month_range = to_month.year() * 12 + to_month.month() as i32
            - (from_month.year() * 12 + from_month.month() as i32)
            + 1;

        if month_range <= 0 {
            Err(InputValueError::custom(format!(
                "`From` date {} has to be before `to` date {}.",
                from_month, to_month
            )))
        } else if month_range <= self.allowed_month_range as i32 {
            Ok(())
        } else {
            Err(InputValueError::custom(format!(
                "Expected maximum range of {} months, got {}.",
                self.allowed_month_range, month_range
            )))
        }
    }
}
