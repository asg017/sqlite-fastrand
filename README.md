# sqlite-fastrandom

A SQLite extension for quickly generating random numbers, booleans, characters, and blobs. **Not cryptographically secure.** Based on [`sqlite-loadable-rs`](https://github.com/asg017/sqlite-loadable-rs) and the [fastrand crate](https://crates.io/crates/fastrand).`

According to my local benchmarks, `fastrand_int64()` is about 2.6x faster than SQLite's `random()`, and `fastrand_blob()` is about 1.6x faster than `randomblob()`. `sqlite-fastrand` also offers a more ergonomic API with custom ranges, seeds, and boolean/character support. However, it yields psuedo-random results and isn't "truly" random.

If your company or organization finds this library useful, consider [supporting my work](#supporting)!

## Usage

```sql
.load ./fastrand0
select fastrand_int(); -- 556823563
select fastrand_int(); -- 363294620
select fastrand_int(); -- -320463573
```

Set a seed for the underlying random number generator, for deterministic values.

```sql
select fastrand_seed_set(1234);
select fastrand_int(); -- -2058591105
select fastrand_int(); -- -211244717
select fastrand_int(); -- -1772832958

select fastrand_seed_set(1234);
select fastrand_int(); -- -2058591105
select fastrand_int(); -- -211244717
select fastrand_int(); -- -1772832958
```

Include `start` and `end` (exclusive) parameters to generate random numbers within a range.

```sql
select fastrand_int(0, 10); -- 0
select fastrand_int(0, 10); -- 9
select fastrand_int(0, 10); -- 6
```

Generate random digits, lowercase/uppercase/alphabetic/alphanumeric characters.

```sql
select fastrand_alphabetic(); -- 's'
select fastrand_alphanumeric(); -- '2'
select fastrand_char(); -- '񠞼'
select fastrand_lowercase(); -- 'g'
select fastrand_uppercase();-- 'M'

select fastrand_digit(16); -- 'c'
```

Generate a random float between 0 and 1.

```sql
select fastrand_double(); -- 0.740834390248454
select fastrand_double(); -- 0.46936608707793
```

### Differences from `random()` and `randomblob()`

The builtin [`random()`](https://www.sqlite.org/lang_corefunc.html#random) and [`randomblob()`](https://www.sqlite.org/lang_corefunc.html#randomblob) are powerful tools that already exist in SQLite standard library, but they can be confusing at times.

For example, the `random()` function returns "_... a pseudo-random integer between -9223372036854775808 and +9223372036854775807_", which are the minimum and maximum values of a 64 bit signed integer.

```sql
select random(); -- 8247412491507365610
select random(); -- 8124278049726255864
```

This may work fine in your use-case, but typically I want a more constrained random number, like any number between `0-100`. This can technically be done with `random()` if you use `abs()` and the modulus `%` operator, but it gets awkward:

```sql
select abs(random()) % 100; -- 96
select abs(random()) % 100; -- 41
```

The `fastrand_int64()` function works the same as `random()` but offers an optional `start` and `end` parameters to specify a range in which the random number should be generated in.

```sql
select fastrand_int64(); -- 5216671854996406003
select fastrand_int64(0, 100); -- 19
```

[`randomblob(N)`](https://www.sqlite.org/lang_corefunc.html#randomblob)

> _The randomblob(N) function return an N-byte blob containing pseudo-random bytes. If N is less than 1 then a 1-byte random blob is returned._

```sql
select hex(randomblob(16)); -- '4E7EFDB9E687EED4F376359986CB695E'
select hex(randomblob(16)); -- 'F6CFF9249E3BD8755E10D6BB3CA81C66'
```

The [`fastrand_blob(N)`](./docs.md#fastrand_blob) function acts in the same way.

```sql
select hex(fastrand_blob(16)); -- 'D86FF5409D3FAD7DBE707580C7E7DE14'
select hex(fastrand_blob(16)); -- 'AB72BFE9480197F487933E8071072D4A'
```

## Installing

| Language       | Install                                                              |                                                                                                                                                                                                     |
| -------------- | -------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Python         | `pip install sqlite-fastrand`                                        | [![PyPI](https://img.shields.io/pypi/v/sqlite-fastrand.svg?color=blue&logo=python&logoColor=white)](https://pypi.org/project/sqlite-fastrand/)                                                      |
| Datasette      | `datasette install datasette-sqlite-fastrand`                        | [![Datasette](https://img.shields.io/pypi/v/datasette-sqlite-fastrand.svg?color=B6B6D9&label=Datasette+plugin&logoColor=white&logo=python)](https://datasette.io/plugins/datasette-sqlite-fastrand) |
| Node.js        | `npm install sqlite-fastrand`                                        | [![npm](https://img.shields.io/npm/v/sqlite-fastrand.svg?color=green&logo=nodedotjs&logoColor=white)](https://www.npmjs.com/package/sqlite-fastrand)                                                |
| Deno           | [`deno.land/x/sqlite_fastrand`](https://deno.land/x/sqlite_fastrand) | [![deno.land/x release](https://img.shields.io/github/v/release/asg017/sqlite-fastrand?color=fef8d2&include_prereleases&label=deno.land%2Fx&logo=deno)](https://deno.land/x/sqlite_fastrand)        |
| Ruby           | `gem install sqlite-fastrand`                                        | ![Gem](https://img.shields.io/gem/v/sqlite-fastrand?color=red&logo=rubygems&logoColor=white)                                                                                                        |
| Github Release |                                                                      | ![GitHub tag (latest SemVer pre-release)](https://img.shields.io/github/v/tag/asg017/sqlite-fastrand?color=lightgrey&include_prereleases&label=Github+release&logo=github)                          |
| Rust           | `cargo add sqlite-fastrand`                                          | [![Crates.io](https://img.shields.io/crates/v/sqlite-fastrand?logo=rust)](https://crates.io/crates/sqlite-fastrand)                                                                                 |

<!--
| Elixir         | [`hex.pm/packages/sqlite_fastrand`](https://hex.pm/packages/sqlite_fastrand) | [![Hex.pm](https://img.shields.io/hexpm/v/sqlite_fastrand?color=purple&logo=elixir)](https://hex.pm/packages/sqlite_fastrand)                                                                       |
| Go             | `go get -u github.com/asg017/sqlite-fastrand/bindings/go`               | [![Go Reference](https://pkg.go.dev/badge/github.com/asg017/sqlite-fastrand/bindings/go.svg)](https://pkg.go.dev/github.com/asg017/sqlite-fastrand/bindings/go)                                     |
-->

The [Releases page](https://github.com/asg017/sqlite-fastrand/releases) contains pre-built binaries for Linux x86_64, MacOS, and Windows.

### As a loadable extension

If you want to use `sqlite-fastrand` as a [Runtime-loadable extension](https://www.sqlite.org/loadext.html), Download the `fastrand0.dylib` (for MacOS), `fastrand0.so` (Linux), or `fastrand0.dll` (Windows) file from a release and load it into your SQLite environment.

> **Note:**
> The `0` in the filename (`fastrand0.dylib`/ `fastrand0.so`/`fastrand0.dll`) denotes the major version of `sqlite-fastrand`. Currently `sqlite-fastrand` is pre v1, so expect breaking changes in future versions.

For example, if you are using the [SQLite CLI](https://www.sqlite.org/cli.html), you can load the library like so:

```sql
.load ./fastrand0
select fastrand_version();
-- v0.1.0
```

Or in Python, using the builtin [sqlite3 module](https://docs.python.org/3/library/sqlite3.html):

```python
import sqlite3
con = sqlite3.connect(":memory:")
con.enable_load_extension(True)
con.load_extension("./fastrand0")
print(con.execute("select fastrand_version()").fetchone())
# ('v0.1.0',)
```

Or in Node.js using [better-sqlite3](https://github.com/WiseLibs/better-sqlite3):

```javascript
const Database = require("better-sqlite3");
const db = new Database(":memory:");
db.loadExtension("./fastrand0");
console.log(db.prepare("select fastrand_version()").get());
// { 'fastrand_version()': 'v0.1.0' }
```

Or with [Datasette](https://datasette.io/):

```
datasette data.db --load-extension ./fastrand0
```

## Supporting

I (Alex 👋🏼) spent a lot of time and energy on this project and [many other open source projects](https://github.com/asg017?tab=repositories&q=&type=&language=&sort=stargazers). If your company or organization uses this library (or you're feeling generous), then please [consider supporting my work](https://alexgarcia.xyz/work.html), or share this project with a friend!

## See also

- [sqlite-xsv](https://github.com/asg017/sqlite-xsv), A SQLite extension for working with CSVs
- [sqlite-loadable](https://github.com/asg017/sqlite-loadable-rs), A framework for writing SQLite extensions in Rust
- [sqlite-http](https://github.com/asg017/sqlite-http), A SQLite extension for making HTTP requests
