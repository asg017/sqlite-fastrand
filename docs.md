# `sqlite-fastrand` Documentation

A full reference to every function and module that `sqlite-fastrand` offers.

As a reminder, `sqlite-fastrand` follows semver and is pre v1, so breaking changes are to be expected.

## API Reference

<h3 name="fastrand_version"><code>fastrand_version()</code></h3>

x

```sql
select fastrand_version(); --
```

<h3 name="fastrand_debug"><code>fastrand_debug()</code></h3>

x

```sql
select fastrand_debug(); --
```

<h3 name="fastrand_seed_get"><code>fastrand_seed_get()</code></h3>

Get the current seed value powering the underlying random number generator. Based on [`Rng.get_seed()`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.get_seed).

```sql
select fastrand_seed_get(); --
```

<h3 name="fastrand_seed_set"><code>fastrand_seed_set()</code></h3>

x. Based on [`Rng.seed()`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.seed).

```sql
select fastrand_seed_set(); --
```

<h3 name="fastrand_double"><code>fastrand_double()</code></h3>

x. Based on [`Rng.f64()`]https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.f64().

```sql
select fastrand_double(); --
```

<h3 name="fastrand_int"><code>fastrand_int()</code></h3>

x. Based on [`Rng.i32()`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.i32).

```sql
select fastrand_int(); --
```

<h3 name="fastrand_int64"><code>fastrand_int64()</code></h3>

x. Based on [`Rng.i64()`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.i64).

```sql
select fastrand_int64(); --
```

<h3 name="fastrand_blob"><code>fastrand_blob()</code></h3>

x. Based on [`Rng.u8()`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.u8).

```sql
select fastrand_blob(); --
```

<h3 name="fastrand_bool"><code>fastrand_bool()</code></h3>

x. Based on [`Rng.bool`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.bool).

```sql
select fastrand_bool(); --
```

<h3 name="fastrand_alphabetic"><code>fastrand_alphabetic()</code></h3>

x. Based on [`Rng.alphabetic`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.alphabetic).

```sql
select fastrand_alphabetic(); --
```

<h3 name="fastrand_alphanumeric"><code>fastrand_alphanumeric()</code></h3>

x. Based on [`Rng.alphanumeric`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.alphanumeric).

```sql
select fastrand_alphanumeric(); --
```

<h3 name="fastrand_char"><code>fastrand_char()</code></h3>

x. Based on [`Rng.`char()](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.char).

```sql
select fastrand_char(); --
```

<h3 name="fastrand_digit"><code>fastrand_digit()</code></h3>

x. Based on [`Rng.digit`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.digit).

```sql
select fastrand_digit(); --
```

<h3 name="fastrand_lowercase"><code>fastrand_lowercase()</code></h3>

x. Based on [`Rng.lowercase()`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.lowercase).

```sql
select fastrand_lowercase(); --
```

<h3 name="fastrand_uppercase"><code>fastrand_uppercase()</code></h3>

x. Based on [`Rng.uppercase()`](https://docs.rs/fastrand/1.8.0/fastrand/struct.Rng.html#method.uppercase).

```sql
select fastrand_uppercase(); --
```
