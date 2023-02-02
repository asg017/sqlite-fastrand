# The `datasette-sqlite-fastrand` Datasette Plugin

`datasette-sqlite-fastrand` is a [Datasette plugin](https://docs.datasette.io/en/stable/plugins.html) that loads the [`sqlite-fastrand`](https://github.com/asg017/sqlite-fastrand) extension in Datasette instances, allowing you to generate and work with [fastrands](https://github.com/fastrand/spec) in SQL.

```
datasette install datasette-sqlite-fastrand
```

See [`docs.md`](../../docs.md) for a full API reference for the fastrand SQL functions.

Alternatively, when publishing Datasette instances, you can use the `--install` option to install the plugin.

```
datasette publish cloudrun data.db --service=my-service --install=datasette-sqlite-fastrand

```
