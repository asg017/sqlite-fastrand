from setuptools import setup

version = {}
with open("datasette_sqlite_fastrand/version.py") as fp:
    exec(fp.read(), version)

VERSION = version['__version__']


setup(
    name="datasette-sqlite-fastrand",
    description="",
    long_description="",
    long_description_content_type="text/markdown",
    author="Alex Garcia",
    url="https://github.com/asg017/sqlite-fastrand",
    project_urls={
        "Issues": "https://github.com/asg017/sqlite-fastrand/issues",
        "CI": "https://github.com/asg017/sqlite-fastrand/actions",
        "Changelog": "https://github.com/asg017/sqlite-fastrand/releases",
    },
    license="MIT License, Apache License, Version 2.0",
    version=VERSION,
    packages=["datasette_sqlite_fastrand"],
    entry_points={"datasette": ["sqlite_fastrand = datasette_sqlite_fastrand"]},
    install_requires=["datasette", "sqlite-fastrand"],
    extras_require={"test": ["pytest"]},
    python_requires=">=3.7",
)