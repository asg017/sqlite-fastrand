<!--- Generated with the deno_generate_package.sh script, don't edit by hand! -->

# `x/sqlite_fastrand` Deno Module

[![Tags](https://img.shields.io/github/release/asg017/sqlite-fastrand)](https://github.com/asg017/sqlite-fastrand/releases)
[![Doc](https://doc.deno.land/badge.svg)](https://doc.deno.land/https/deno.land/x/sqlite-fastrand@0.2.1/mod.ts)

The [`sqlite-fastrand`](https://github.com/asg017/sqlite-fastrand) SQLite extension is available to Deno developers with the [`x/sqlite_fastrand`](https://deno.land/x/sqlite_fastrand) Deno module. It works with [`x/sqlite3`](https://deno.land/x/sqlite3), the fastest and native Deno SQLite3 module.

```js
import { Database } from "https://deno.land/x/sqlite3@0.8.0/mod.ts";
import * as sqlite_fastrand from "https://deno.land/x/sqlite_fastrand@v0.2.1/mod.ts";

const db = new Database(":memory:");

  db.enableLoadExtension = true;
  db.loadExtension(sqlite_fastrand.getLoadablePath());

  const [version] = db
    .prepare("select fastrand_version()")
    .value<[string]>()!;

  console.log(version);

```

Like `x/sqlite3`, `x/sqlite_fastrand` requires network and filesystem permissions to download and cache the pre-compiled SQLite extension for your machine. Though `x/sqlite3` already requires `--allow-ffi` and `--unstable`, so you might as well use `--allow-all`/`-A`.

```bash
deno run -A --unstable <file>
```

`x/sqlite_fastrand` does not work with [`x/sqlite`](https://deno.land/x/sqlite@v3.7.0), which is a WASM-based Deno SQLite module that does not support loading extensions.
