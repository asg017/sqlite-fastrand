use sqlite_loadable::api::ValueType;
use sqlite_loadable::{api, define_scalar_function, define_scalar_function_with_aux, Result};
use sqlite_loadable::{prelude::*, Error};

use fastrand::Rng;
use std::cell::{Ref, RefCell};
use std::iter::repeat_with;
use std::ops::Range;
use std::rc::Rc;

enum BtwnValues<'a> {
    Range(&'a *mut sqlite3_value, &'a *mut sqlite3_value),
    From(&'a *mut sqlite3_value),
    To(&'a *mut sqlite3_value),
    Full,
}

fn calculate_btwn_values(values: &[*mut sqlite3_value]) -> Result<BtwnValues> {
    let a = values.get(0);
    let b = values.get(1);
    return match (a, b) {
        (Some(a), Some(b)) => {
            match (
                api::value_type(a) == ValueType::Null,
                api::value_type(b) == ValueType::Null,
            ) {
                (true, true) => Ok(BtwnValues::Full),
                (false, true) => Ok(BtwnValues::From(a)),
                (false, false) => Ok(BtwnValues::Range(a, b)),
                (true, false) => Ok(BtwnValues::To(b)),
            }
        }
        (None, None) => Ok(BtwnValues::Full),
        (None, Some(_)) => Err(Error::new_message(
            "sqlite-fastrand internal error: argv[1] provided but not argv[0]",
        )),
        (Some(a), None) => {
            if api::value_type(a) == ValueType::Null {
                Ok(BtwnValues::Full)
            } else {
                Ok(BtwnValues::From(a))
            }
        }
    };
}

pub fn fastrand_version(
    context: *mut sqlite3_context,
    _values: &[*mut sqlite3_value],
) -> Result<()> {
    api::result_text(context, format!("v{}", env!("CARGO_PKG_VERSION")))?;
    Ok(())
}

pub fn fastrand_debug(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_text(
        context,
        format!(
            "Version: v{}
Source: {}
",
            env!("CARGO_PKG_VERSION"),
            env!("GIT_HASH")
        ),
    )?;
    Ok(())
}

fn rng_borrow_or_err(rng: &Rc<RefCell<Rng>>) -> Result<Ref<Rng>> {
    match rng.try_borrow() {
        Ok(rng) => Ok(rng),
        Err(err) => Err(Error::new_message(
            format!("internal sqlite-fastrand error, could not borrow shared RNG: {err}").as_str(),
        )),
    }
}

pub fn fastrand_seed_set(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
    rng: &Rc<RefCell<Rng>>,
) -> Result<()> {
    let seed: u64 = api::value_int64(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("expected seed as 1st argument"))?,
    )
    .try_into()
    .map_err(|e| {
        Error::new_message(format!("seed must be an usigned 64 bit integer: {e}").as_str())
    })?;
    rng.borrow_mut().seed(seed);
    api::result_bool(context, true);
    Ok(())
}

pub fn fastrand_seed_get(
    context: *mut sqlite3_context,
    _values: &[*mut sqlite3_value],
    rng: &Rc<RefCell<Rng>>,
) -> Result<()> {
    api::result_text(context, rng_borrow_or_err(rng)?.get_seed().to_string())?;
    Ok(())
}

pub fn fastrand_int(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
    rng: &Rc<RefCell<Rng>>,
) -> Result<()> {
    let i = match calculate_btwn_values(values)? {
        BtwnValues::Range(start, end) => {
            let range: Range<i32> = std::ops::Range {
                start: api::value_int(start),
                end: api::value_int(end),
            };
            if range.is_empty() {
                return Err("fuk".into());
            }
            rng_borrow_or_err(rng)?.i32(range)
        }
        BtwnValues::From(start) => {
            let range = std::ops::RangeFrom {
                start: api::value_int(start),
            };
            rng_borrow_or_err(rng)?.i32(range)
        }
        BtwnValues::To(end) => {
            let range = std::ops::RangeTo {
                end: api::value_int(end),
            };
            rng_borrow_or_err(rng)?.i32(range)
        }
        BtwnValues::Full => rng_borrow_or_err(rng)?.i32(..),
    };
    api::result_int(context, i);
    Ok(())
}

pub fn fastrand_int64(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
    rng: &Rc<RefCell<Rng>>,
) -> Result<()> {
    let i = match calculate_btwn_values(values)? {
        BtwnValues::Range(start, end) => {
            let range: Range<i64> = std::ops::Range {
                start: api::value_int64(start),
                end: api::value_int64(end),
            };
            if range.is_empty() {
                return Err("fuk".into());
            }
            rng_borrow_or_err(rng)?.i64(range)
        }
        BtwnValues::From(start) => {
            let range = std::ops::RangeFrom {
                start: api::value_int64(start),
            };
            rng_borrow_or_err(rng)?.i64(range)
        }
        BtwnValues::To(end) => {
            let range = std::ops::RangeTo {
                end: api::value_int64(end),
            };
            rng_borrow_or_err(rng)?.i64(range)
        }
        BtwnValues::Full => rng_borrow_or_err(rng)?.i64(..),
    };
    api::result_int64(context, i);
    Ok(())
}

pub fn fastrand_double(
    context: *mut sqlite3_context,
    _values: &[*mut sqlite3_value],
    rng: &Rc<RefCell<Rng>>,
) -> Result<()> {
    api::result_double(context, rng_borrow_or_err(rng)?.f64());
    Ok(())
}

pub fn fastrand_blob(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
    rng: &Rc<RefCell<Rng>>,
) -> Result<()> {
    let n: usize = api::value_int64(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("expected N as 1st argument"))?,
    ) as usize;
    let rng = rng_borrow_or_err(rng)?;
    let bytes: Vec<u8> = repeat_with(|| rng.u8(..)).take(n).collect();
    api::result_blob(context, bytes.as_slice());
    Ok(())
}

pub fn fastrand_bool(
    context: *mut sqlite3_context,
    _values: &[*mut sqlite3_value],
    rng: &Rc<RefCell<Rng>>,
) -> Result<()> {
    api::result_bool(context, rng_borrow_or_err(rng)?.bool());
    Ok(())
}

pub fn fastrand_alphabetic(
    context: *mut sqlite3_context,
    _values: &[*mut sqlite3_value],
    rng: &Rc<RefCell<Rng>>,
) -> Result<()> {
    api::result_text(context, rng_borrow_or_err(rng)?.alphabetic().to_string())?;
    Ok(())
}

pub fn fastrand_alphanumeric(
    context: *mut sqlite3_context,
    _values: &[*mut sqlite3_value],
    rng: &Rc<RefCell<Rng>>,
) -> Result<()> {
    api::result_text(context, rng_borrow_or_err(rng)?.alphanumeric().to_string())?;
    Ok(())
}

pub fn fastrand_lowercase(
    context: *mut sqlite3_context,
    _values: &[*mut sqlite3_value],
    rng: &Rc<RefCell<Rng>>,
) -> Result<()> {
    api::result_text(context, rng_borrow_or_err(rng)?.lowercase().to_string())?;
    Ok(())
}

pub fn fastrand_uppercase(
    context: *mut sqlite3_context,
    _values: &[*mut sqlite3_value],
    rng: &Rc<RefCell<Rng>>,
) -> Result<()> {
    api::result_text(context, rng_borrow_or_err(rng)?.uppercase().to_string())?;
    Ok(())
}

pub fn fastrand_digit(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
    rng: &Rc<RefCell<Rng>>,
) -> Result<()> {
    let base: u32 = api::value_int64(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("expected base as 1st argument"))?,
    )
    .try_into()
    .map_err(|e| {
        Error::new_message(format!("base must be an unsigned 32-bit integer: {e}").as_str())
    })?;
    match base {
        1..=35 => api::result_text(context, rng_borrow_or_err(rng)?.digit(base).to_string())?,
        _ => {
            return Err(Error::new_message(
                "base must be greater than 0 and less than 26",
            ))
        }
    }

    Ok(())
}

#[sqlite_entrypoint]
pub fn sqlite3_fastrand_init(db: *mut sqlite3) -> Result<()> {
    let flags = FunctionFlags::empty();
    let rng = Rc::new(RefCell::new(Rng::new()));

    define_scalar_function(
        db,
        "fastrand_version",
        0,
        fastrand_version,
        FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC,
    )?;
    define_scalar_function(
        db,
        "fastrand_debug",
        0,
        fastrand_debug,
        FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC,
    )?;

    define_scalar_function_with_aux(
        db,
        "fastrand_seed_get",
        0,
        fastrand_seed_get,
        flags,
        Rc::clone(&rng),
    )?;
    define_scalar_function_with_aux(
        db,
        "fastrand_seed_set",
        1,
        fastrand_seed_set,
        flags,
        Rc::clone(&rng),
    )?;

    define_scalar_function_with_aux(db, "fastrand_int", 0, fastrand_int, flags, Rc::clone(&rng))?;
    define_scalar_function_with_aux(db, "fastrand_int", 1, fastrand_int, flags, Rc::clone(&rng))?;
    define_scalar_function_with_aux(db, "fastrand_int", 2, fastrand_int, flags, Rc::clone(&rng))?;
    define_scalar_function_with_aux(
        db,
        "fastrand_int64",
        0,
        fastrand_int64,
        flags,
        Rc::clone(&rng),
    )?;
    define_scalar_function_with_aux(
        db,
        "fastrand_int64",
        1,
        fastrand_int64,
        flags,
        Rc::clone(&rng),
    )?;
    define_scalar_function_with_aux(
        db,
        "fastrand_int64",
        2,
        fastrand_int64,
        flags,
        Rc::clone(&rng),
    )?;
    define_scalar_function_with_aux(
        db,
        "fastrand_double",
        0,
        fastrand_double,
        flags,
        Rc::clone(&rng),
    )?;
    define_scalar_function_with_aux(
        db,
        "fastrand_blob",
        1,
        fastrand_blob,
        flags,
        Rc::clone(&rng),
    )?;

    define_scalar_function_with_aux(
        db,
        "fastrand_bool",
        0,
        fastrand_bool,
        flags,
        Rc::clone(&rng),
    )?;

    define_scalar_function_with_aux(
        db,
        "fastrand_alphabetic",
        0,
        fastrand_alphabetic,
        flags,
        Rc::clone(&rng),
    )?;
    define_scalar_function_with_aux(
        db,
        "fastrand_alphanumeric",
        0,
        fastrand_alphanumeric,
        flags,
        Rc::clone(&rng),
    )?;
    define_scalar_function_with_aux(
        db,
        "fastrand_uppercase",
        0,
        fastrand_uppercase,
        flags,
        Rc::clone(&rng),
    )?;
    define_scalar_function_with_aux(
        db,
        "fastrand_lowercase",
        0,
        fastrand_lowercase,
        flags,
        Rc::clone(&rng),
    )?;

    define_scalar_function_with_aux(
        db,
        "fastrand_digit",
        1,
        fastrand_digit,
        flags,
        Rc::clone(&rng),
    )?;
    Ok(())
}
