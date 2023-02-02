from datasette.app import Datasette
import pytest


@pytest.mark.asyncio
async def test_plugin_is_installed():
    datasette = Datasette(memory=True)
    response = await datasette.client.get("/-/plugins.json")
    assert response.status_code == 200
    installed_plugins = {p["name"] for p in response.json()}
    assert "datasette-sqlite-fastrand" in installed_plugins

@pytest.mark.asyncio
async def test_sqlite_fastrand_functions():
    datasette = Datasette(memory=True)
    response = await datasette.client.get("/_memory.json?sql=select+fastrand_version(),fastrand()")
    assert response.status_code == 200
    fastrand_version, fastrand = response.json()["rows"][0]
    assert fastrand_version[0] == "v"
    assert len(fastrand) == 26