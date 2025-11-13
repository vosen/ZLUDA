#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("c_bindings.rs");

pub const MODEL_STATUS_NOTSET: HighsInt = 0;
pub const MODEL_STATUS_LOAD_ERROR: HighsInt = 1;
pub const MODEL_STATUS_MODEL_ERROR: HighsInt = 2;
pub const MODEL_STATUS_PRESOLVE_ERROR: HighsInt = 3;
pub const MODEL_STATUS_SOLVE_ERROR: HighsInt = 4;
pub const MODEL_STATUS_POSTSOLVE_ERROR: HighsInt = 5;
pub const MODEL_STATUS_MODEL_EMPTY: HighsInt = 6;
pub const MODEL_STATUS_OPTIMAL: HighsInt = 7;
pub const MODEL_STATUS_INFEASIBLE: HighsInt = 8;
pub const MODEL_STATUS_UNBOUNDED_OR_INFEASIBLE: HighsInt = 9;
pub const MODEL_STATUS_UNBOUNDED: HighsInt = 10;
pub const MODEL_STATUS_OBJECTIVE_BOUND: HighsInt = 11;
pub const MODEL_STATUS_OBJECTIVE_TARGET: HighsInt = 12;
pub const MODEL_STATUS_REACHED_TIME_LIMIT: HighsInt = 13;
pub const MODEL_STATUS_REACHED_ITERATION_LIMIT: HighsInt = 14;
pub const MODEL_STATUS_UNKNOWN: HighsInt = 15;
pub const MODEL_STATUS_MIN: HighsInt = MODEL_STATUS_NOTSET;
pub const MODEL_STATUS_MAX: HighsInt = MODEL_STATUS_UNKNOWN;

pub const STATUS_OK: HighsInt = 0;
pub const STATUS_WARNING: HighsInt = 1;
pub const STATUS_ERROR: HighsInt = -1;

pub const MATRIX_FORMAT_NONE: HighsInt = 0;
pub const MATRIX_FORMAT_COLUMN_WISE: HighsInt = 1;
pub const MATRIX_FORMAT_ROW_WISE: HighsInt = 2;

pub const OBJECTIVE_SENSE_MINIMIZE: HighsInt = 1;
pub const OBJECTIVE_SENSE_MAXIMIZE: HighsInt = -1;
