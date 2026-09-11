pub struct Minutes(pub i32);
pub struct Hours(pub i32);
pub struct Days(pub i32);

impl From<Minutes> for Hours {
    fn from(minutes: Minutes) -> Hours {
        // Implement the minute to hour conversion here
        Hours(minutes.0 / 60)
    }
}

// TODO: implement from hours to days
impl From<Hours> for Days {
    fn from(hours: Hours) -> Self {
        Days(hours.0 / 24)
    }
}

// TODO: implement from minutes to days
impl From<Minutes> for Days {
    fn from(minutes: Minutes) -> Self {
        Days(minutes.0 / 60 / 24)
    }
}

// TODO: implement from days to hours
impl From<Days> for Hours {
    fn from(days: Days) -> Self {
        Hours(days.0 * 24)
    }
}

// Example usage
pub fn main() {
    let minutes = Minutes(60);
    let hours: Hours = minutes.into();
    assert_eq!(hours.0, 1);
}
