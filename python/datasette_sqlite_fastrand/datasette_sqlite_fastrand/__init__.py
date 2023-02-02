from datasette import hookimpl
import sqlite_fastrand

@hookimpl
def prepare_connection(conn):
    conn.enable_load_extension(True)
    sqlite_fastrand.load(conn)
    conn.enable_load_extension(False)