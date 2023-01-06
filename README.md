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
