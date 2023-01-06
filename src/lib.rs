use sqlite_loadable::api::ValueType;
use sqlite_loadable::{api, define_scalar_function, define_scalar_function_with_aux, Result};
use sqlite_loadable::{prelude::*, Error};

use std::cell::RefCell;
use std::rc::Rc;

pub fn fastrand_bool(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_bool(context, fastrand::bool());
    Ok(())
}

pub fn fastrand_i8(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_int(context, fastrand::i8(..).into());
    Ok(())
}

enum BtwnValues<'a> {
    Range(&'a *mut sqlite3_value, &'a *mut sqlite3_value),
    From(&'a *mut sqlite3_value),
    To(&'a *mut sqlite3_value),
    Full,
}

fn calculate_btwn_values(values: &[*mut sqlite3_value]) -> BtwnValues {
    let a = values.get(0).unwrap();
    let b = values.get(1);
    let a_null = api::value_type(a) == ValueType::Null;
    match (b, a_null) {
        (Some(b), true) => BtwnValues::To(b),
        (Some(b), false) => BtwnValues::Range(a, b),
        (None, true) => BtwnValues::Full,
        (None, false) => BtwnValues::From(a),
    }
}

static OOR_I8: &'static str = "asdf {}";
use std::ops::{Range, RangeBounds};
pub fn fastrand_i8_btwn(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
) -> Result<()> {
    let i = match calculate_btwn_values(values) {
        BtwnValues::Range(start, end) => {
            let range: Range<i8> = std::ops::Range {
                start: api::value_int(start)
                    .try_into()
                    .map_err(|e| Error::new_message("asdf"))?,
                end: api::value_int(end).try_into().unwrap(),
            };
            if range.is_empty() {
                return Err("fuk".into());
            }
            fastrand::i8(range)
        }
        BtwnValues::From(start) => {
            let range = std::ops::RangeFrom {
                start: i8::try_from(api::value_int(start)).unwrap(),
            };
            fastrand::i8(range)
        }
        BtwnValues::To(end) => {
            let range = std::ops::RangeTo {
                end: i8::try_from(api::value_int(end)).unwrap(),
            };
            fastrand::i8(range)
        }
        BtwnValues::Full => fastrand::i8(..),
    };
    api::result_int(context, i.into());

    Ok(())
}
pub fn fastrand_u8(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_int64(context, fastrand::u8(..).into());
    Ok(())
}

pub fn fastrand_i16(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_int(context, fastrand::i16(..).into());
    Ok(())
}
pub fn fastrand_u16(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_int64(context, fastrand::u16(..).into());
    Ok(())
}

pub fn fastrand_i32(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_int(context, fastrand::i32(..));
    Ok(())
}
pub fn fastrand_u32(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_int64(context, fastrand::u32(..).into());
    Ok(())
}

pub fn fastrand_i64(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_int64(context, fastrand::i64(..));
    Ok(())
}
pub fn fastrand_u64(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_text(context, fastrand::u64(..).to_string())?;
    Ok(())
}

pub fn fastrand_i128(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_text(context, fastrand::i128(..).to_string())?;
    Ok(())
}
pub fn fastrand_u128(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_text(context, fastrand::u128(..).to_string())?;
    Ok(())
}

pub fn fastrand_f32(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_double(context, fastrand::f32().into());
    Ok(())
}

pub fn fastrand_f64(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_double(context, fastrand::f64());
    Ok(())
}

pub fn fastrand_char(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_text(context, fastrand::char(..).to_string())?;
    Ok(())
}
pub fn fastrand_alphabetic(
    context: *mut sqlite3_context,
    _values: &[*mut sqlite3_value],
) -> Result<()> {
    api::result_text(context, fastrand::alphabetic().to_string())?;
    Ok(())
}

pub fn fastrand_alphanumeric(
    context: *mut sqlite3_context,
    _values: &[*mut sqlite3_value],
) -> Result<()> {
    api::result_text(context, fastrand::alphanumeric().to_string())?;
    Ok(())
}

pub fn fastrand_lowercase(
    context: *mut sqlite3_context,
    _values: &[*mut sqlite3_value],
) -> Result<()> {
    api::result_text(context, fastrand::lowercase().to_string())?;
    Ok(())
}

pub fn fastrand_uppercase(
    context: *mut sqlite3_context,
    _values: &[*mut sqlite3_value],
) -> Result<()> {
    api::result_text(context, fastrand::uppercase().to_string())?;
    Ok(())
}

pub fn fastrand_digit(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let base = api::value_int64(values.get(0).unwrap());
    api::result_text(
        context,
        fastrand::digit(base.try_into().unwrap()).to_string(),
    )?;
    Ok(())
}

pub fn fastrand_seed(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_text(context, fastrand::get_seed().to_string())?;
    Ok(())
}

use std::iter::repeat_with;
pub fn fastrand_blob(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let n = api::value_int64(values.get(0).unwrap());
    //let bytes: Vec<u8> = (0..n).map(|_| fastrand::u8(..)).collect();
    //api::result_blob(context, bytes.as_slice());
    let rng = fastrand::Rng::new();
    rng.seed(fastrand::get_seed());
    let bytes: Vec<u8> = repeat_with(|| rng.u8(..))
        .take(n.try_into().unwrap())
        .collect();
    api::result_blob(context, bytes.as_slice());
    Ok(())
}

pub fn fastrand_i64x(
    context: *mut sqlite3_context,
    _values: &[*mut sqlite3_value],
    rng: &Rc<RefCell<fastrand::Rng>>,
) -> Result<()> {
    api::result_int64(context, rng.borrow().i64(..));
    Ok(())
}

pub fn fastrand_i32x(
    context: *mut sqlite3_context,
    _values: &[*mut sqlite3_value],
    rng: &Rc<RefCell<fastrand::Rng>>,
) -> Result<()> {
    api::result_int(context, rng.borrow().i32(..));
    Ok(())
}

pub fn fastrand_set_seed(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
    rng: &Rc<RefCell<fastrand::Rng>>,
) -> Result<()> {
    let seed = api::value_int64(values.get(0).unwrap());
    rng.borrow_mut().seed(seed.try_into().unwrap());
    api::result_bool(context, true);
    Ok(())
}
#[sqlite_entrypoint]
pub fn sqlite3_fastrand_init(db: *mut sqlite3) -> Result<()> {
    let flags = FunctionFlags::empty();
    let rng = fastrand::Rng::new();
    let rng = Rc::new(RefCell::new(fastrand::Rng::new()));

    define_scalar_function(db, "fastrand_bool", 0, fastrand_bool, flags)?;

    define_scalar_function(db, "fastrand_char", 0, fastrand_char, flags)?;
    define_scalar_function(db, "fastrand_alphabetic", 0, fastrand_alphabetic, flags)?;
    define_scalar_function(db, "fastrand_alphanumeric", 0, fastrand_alphanumeric, flags)?;
    define_scalar_function(db, "fastrand_uppercase", 0, fastrand_uppercase, flags)?;
    define_scalar_function(db, "fastrand_lowercase", 0, fastrand_lowercase, flags)?;

    define_scalar_function(db, "fastrand_i8", 0, fastrand_i8, flags)?;
    define_scalar_function(db, "fastrand_i8", 1, fastrand_i8_btwn, flags)?;
    define_scalar_function(db, "fastrand_i8", 2, fastrand_i8_btwn, flags)?;

    define_scalar_function(db, "fastrand_u8", 0, fastrand_u8, flags)?;

    define_scalar_function(db, "fastrand_i16", 0, fastrand_i16, flags)?;
    define_scalar_function(db, "fastrand_u16", 0, fastrand_u16, flags)?;

    define_scalar_function(db, "fastrand_i32", 0, fastrand_i32, flags)?;
    define_scalar_function(db, "fastrand_u32", 0, fastrand_u32, flags)?;

    define_scalar_function_with_aux(db, "fastrand_int", 0, fastrand_i32x, flags, Rc::clone(&rng))?;
    define_scalar_function_with_aux(
        db,
        "fastrand_int64",
        0,
        fastrand_i64x,
        flags,
        Rc::clone(&rng),
    )?;
    define_scalar_function_with_aux(
        db,
        "fastrand_set_seed",
        1,
        fastrand_set_seed,
        flags,
        Rc::clone(&rng),
    )?;

    define_scalar_function(db, "fastrand_i64", 0, fastrand_i64, flags)?;
    define_scalar_function(db, "fastrand_u64", 0, fastrand_u64, flags)?;

    define_scalar_function(db, "fastrand_i128", 0, fastrand_i128, flags)?;
    define_scalar_function(db, "fastrand_u128", 0, fastrand_u128, flags)?;

    define_scalar_function(db, "fastrand_float", 0, fastrand_f32, flags)?;
    define_scalar_function(db, "fastrand_f32", 0, fastrand_f32, flags)?;
    define_scalar_function(db, "fastrand_double", 0, fastrand_f64, flags)?;
    define_scalar_function(db, "fastrand_f64", 0, fastrand_f64, flags)?;

    define_scalar_function(db, "fastrand_digit", 1, fastrand_digit, flags)?;

    define_scalar_function(db, "fastrand_seed", 0, fastrand_seed, flags)?;
    define_scalar_function(db, "fastrand_blob", 1, fastrand_blob, flags)?;
    Ok(())
}
