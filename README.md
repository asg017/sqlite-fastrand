# sqlite-fastrandom

A fast and performant SQLite extension for regular expressions. Based on [`sqlite-loadable-rs`](https://github.com/asg017/sqlite-loadable-rs), and the [regex crate](https://crates.io/crates/regex).

If your company or organization finds this library useful, consider [supporting my work](#supporting)!

## Supporting

I (Alex 👋🏼) spent a lot of time and energy on this project and [many other open source projects](https://github.com/asg017?tab=repositories&q=&type=&language=&sort=stargazers). If your company or organization uses this library (or you're feeling generous), then please [consider supporting my work](https://alexgarcia.xyz/work.html), or share this project with a friend!

## See also

- [sqlite-xsv](https://github.com/asg017/sqlite-xsv), A SQLite extension for working with CSVs
- [sqlite-loadable](https://github.com/asg017/sqlite-loadable-rs), A framework for writing SQLite extensions in Rust
- [sqlite-http](https://github.com/asg017/sqlite-http), A SQLite extension for making HTTP requests

## How the SQLite `random()` works

[`random()`](https://www.sqlite.org/lang_corefunc.html#random)

> _The random() function returns a pseudo-random integer between -9223372036854775808 and +9223372036854775807._

```sql
select random(); --
select random(); --
select random(); --
```

```
select abs(random()) % 10;
```

[`randomblob(N)`](https://www.sqlite.org/lang_corefunc.html#randomblob)

> _The randomblob(N) function return an N-byte blob containing pseudo-random bytes. If N is less than 1 then a 1-byte random blob is returned._

### Downsides
