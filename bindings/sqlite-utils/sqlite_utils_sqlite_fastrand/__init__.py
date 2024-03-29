from sqlite_utils import hookimpl
import sqlite_fastrand

from sqlite_utils_sqlite_fastrand.version import __version_info__, __version__


@hookimpl
def prepare_connection(conn):
    conn.enable_load_extension(True)
    sqlite_fastrand.load(conn)
    conn.enable_load_extension(False)
