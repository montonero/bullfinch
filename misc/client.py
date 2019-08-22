#!/usr/bin/env python3

import json
import asyncio
import aiohttp

async def req():
    resp = await aiohttp.ClientSession().request(
        #"get", 'http://localhost:8088/api/v1/links/1',
        "get", 'http://localhost:8088/api/v1/links/1',
        #data=json.dumps({"domain": "http://google.com", "action": "Links"}),
        headers={"content-type": "application/json"})
    print(str(resp))
    print(await resp.text())
    assert 200 == resp.status

async def req2():
    resp = await aiohttp.ClientSession().request(
        "get", 'http://localhost:8088/api/v1/domains',
        headers={"content-type": "application/json"})
    print(str(resp))
    print(await resp.text())
    assert 200 == resp.status


asyncio.get_event_loop().run_until_complete(req())

asyncio.get_event_loop().run_until_complete(req2())