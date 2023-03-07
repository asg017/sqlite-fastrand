# `sqlite-fastrand` Documentation

A full reference to every function and module that `sqlite-fastrand` offers.

As a reminder, `sqlite-fastrand` follows semver and is pre v1, so breaking changes are to be expected.

## API Reference

<h3 name="fastrand_version"><code>fastrand_version()</code></h3>

Returns the semver version string of the current version of `sqlite-fastrand`.

```sql
select fastrand_version(); -- "v0.1.0"
```

<h3 name="fastrand_debug"><code>fastrand_debug()</code></h3>

Returns a debug string of various info about `sqlite-fastrand`, including
the version string, build date, and commit hash.

```sql
select fastrand_debug();
'Version: v0.1.0
Source: 247dca8f4cea1abdc30ed3e852c3e5b71374c177'
```

<h3 name="fastrand_seed_get"><code>fastrand_seed_get()</code></h3>

Gets the current seed value powering the underlying random number generator. Returns text, because the seed is an unsigned 64 bit integer, larger than SQLite's max integer size. Based on [`Rng.get_seed()`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.get_seed).

```sql
select fastrand_seed_get(); -- '3362862862133652073'
```

<h3 name="fastrand_seed_set"><code>fastrand_seed_set()</code></h3>

Set the seed value of the underlying random number generator. Only i64 values allows. Returns 1 if succesful. Based on [`Rng.seed()`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.seed).

```sql
select fastrand_seed_set(); -- 1
```

<h3 name="fastrand_double"><code>fastrand_double()</code></h3>

Returns a random number between 0 and 1. Based on [`Rng.f64()`]https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.f64().

```sql
select fastrand_double(); -- 0.644268998061438
select fastrand_double(); -- 0.810443374870184
```

<h3 name="fastrand_int"><code>fastrand_int()</code></h3>

Returns a random 32 bit integer between `-2_147_483_648` and `2_147_483_647`. Based on [`Rng.i32()`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.i32).

```sql
select fastrand_int(); -- 1321083668
select fastrand_int(); -- 1579705906
```

<h3 name="fastrand_int64"><code>fastrand_int64()</code></h3>

Returns a random 64 bit integer between `-9_223_372_036_854_775_808` and `9_223_372_036_854_775_807`. Based on [`Rng.i64()`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.i64).

```sql
select fastrand_int64(); -- -6587368321689545426
select fastrand_int64(); -- 7259195115703397710
```

<h3 name="fastrand_blob"><code>fastrand_blob(N)</code></h3>

Returns a blob of length `N` with random bytes. Based on [`Rng.u8()`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.u8).

```sql
select fastrand_blob(4); -- X'bc71ef5b'
select fastrand_blob(12); -- X'22fce3c0c809c20b0c54251e'
```

<h3 name="fastrand_bool"><code>fastrand_bool()</code></h3>

Returns a random boolean value, `0` or `1`. Based on [`Rng.bool`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.bool).

```sql
select fastrand_bool(); -- 1
select fastrand_bool(); -- 0
```

<h3 name="fastrand_alphabetic"><code>fastrand_alphabetic()</code></h3>

Returns a random alphabetic character, within `a-z` and `A-Z`. Based on [`Rng.alphabetic`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.alphabetic).

```sql
select fastrand_alphabetic(); -- 'Q'
select fastrand_alphabetic(); -- 'b'
```

<h3 name="fastrand_alphanumeric"><code>fastrand_alphanumeric()</code></h3>

Returns a random alphanumeric character, within `a-z`, `A-Z`, or `0-9`. Based on [`Rng.alphanumeric`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.alphanumeric).

```sql
select fastrand_alphanumeric(); -- 'U'
select fastrand_alphanumeric(); -- '4'
select fastrand_alphanumeric(); -- 'i'
```

<h3 name="fastrand_digit"><code>fastrand_digit(base)</code></h3>

Returns a random digit in the given `base`. The `base` must be greater than 0 and less than 36. Based on [`Rng.digit`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.digit).

```sql
select fastrand_digit(4); -- '1'
select fastrand_digit(35); -- 'v'
```

<h3 name="fastrand_lowercase"><code>fastrand_lowercase()</code></h3>

Returns a random character within `a-z`. Based on [`Rng.lowercase()`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.lowercase).

```sql
select fastrand_lowercase(); -- 's'
select fastrand_lowercase(); -- 'p'
```

<h3 name="fastrand_uppercase"><code>fastrand_uppercase()</code></h3>

Returns a random character within `A-Z`. Based on [`Rng.uppercase()`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.uppercase).

```sql
select fastrand_uppercase(); -- 'E'
select fastrand_uppercase(); -- 'J'
```
