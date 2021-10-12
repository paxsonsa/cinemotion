#!/usr/bin/env python
# Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
# Licensed under the MIT License. See LICENSE file in the project root for full license information.
""" client.py """
import asyncio
import time
import websockets


async def hello():
    while True:
        try:
            async with websockets.connect("ws://localhost:7766") as websocket:
                print("connected....sending...")
                await websocket.send("Hello world!")
                print("sent")
                # await websocket.recv()

        except Exception as error:
            print(f"error: {error}")

        time.sleep(3)


asyncio.run(hello())
