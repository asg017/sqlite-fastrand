.load target/debug/libfastrand0

.header on
.mode box
.bail on

select fastrand_bool(), fastrand_bool();
select fastrand_i32(), fastrand_i32();
select fastrand_i64(), fastrand_i64();

select fastrand_seed();


select fastrand_bool(), fastrand_bool();

select fastrand_char(), fastrand_char();
select fastrand_alphabetic(), fastrand_alphabetic();
select fastrand_alphanumeric(), fastrand_alphanumeric();
select fastrand_uppercase(), fastrand_uppercase();
select fastrand_lowercase(), fastrand_lowercase();

select fastrand_i8(), fastrand_i8();
select fastrand_u8(), fastrand_u8();

select fastrand_i16(), fastrand_i16();
select fastrand_u16(), fastrand_u16();

select fastrand_int(), fastrand_int();
select fastrand_i32(), fastrand_i32();
select fastrand_u32(), fastrand_u32();

select fastrand_int64(), fastrand_int64();
select fastrand_i64(), fastrand_i64();
select fastrand_u64(), fastrand_u64();

select fastrand_i128(), fastrand_i128();
select fastrand_u128(), fastrand_u128();

select fastrand_float(), fastrand_float();
select fastrand_f32(), fastrand_f32();
select fastrand_double(), fastrand_double();
select fastrand_f64(), fastrand_f64();

select fastrand_digit(1), fastrand_digit(2);
select fastrand_seed(), fastrand_seed();

select length(fastrand_blob(20)), hex(fastrand_blob(8));




select fastrand_set_seed(0x400);
select fastrand_int() == 881574422;
select fastrand_int() == 654116233;

select fastrand_set_seed(0x400);
select fastrand_int() == 881574422;
select fastrand_int() == 654116233;


select fastrand_i8(1, 2);
