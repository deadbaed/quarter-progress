use super::{Date, IncompleteQuarter};

/// See https://en.wikipedia.org/wiki/Calendar_year#Quarters
pub const QUARTERS: &[IncompleteQuarter] = &[QUARTER_1, QUARTER_2, QUARTER_3, QUARTER_4];

const QUARTER_1: IncompleteQuarter = IncompleteQuarter {
    number: 1,
    start: Date {
        month: 1,
        day: 1,
        hour: 0,
        min: 0,
        sec: 0,
    },
    end: Date {
        month: 3,
        day: 31,
        hour: 23,
        min: 59,
        sec: 59,
    },
};

const QUARTER_2: IncompleteQuarter = IncompleteQuarter {
    number: 2,
    start: Date {
        month: 4,
        day: 1,
        hour: 0,
        min: 0,
        sec: 0,
    },
    end: Date {
        month: 6,
        day: 30,
        hour: 23,
        min: 59,
        sec: 59,
    },
};

const QUARTER_3: IncompleteQuarter = IncompleteQuarter {
    number: 3,
    start: Date {
        month: 7,
        day: 1,
        hour: 0,
        min: 0,
        sec: 0,
    },
    end: Date {
        month: 9,
        day: 30,
        hour: 23,
        min: 59,
        sec: 59,
    },
};

const QUARTER_4: IncompleteQuarter = IncompleteQuarter {
    number: 4,
    start: Date {
        month: 10,
        day: 1,
        hour: 0,
        min: 0,
        sec: 0,
    },
    end: Date {
        month: 12,
        day: 31,
        hour: 23,
        min: 59,
        sec: 59,
    },
};
