#!/usr/bin/env python
# Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
# Licensed under the MIT License. See LICENSE file in the project root for full license information.
""" client.py """
import asyncio
import time
import websockets

import test_client.messages_pb2 as messages

async def hello():
    while True:
        try:

            async with websockets.connect("ws://localhost:8080") as websocket:
                print("connected....sending...")
                m = messages.Message()
                m.session_start.session_properties.api_version = "9999"
                m.session_start.session_properties.session_id = "mysession"
                await websocket.send(m.SerializeToString())
                print("sent")

                string = await websocket.recv()
                print(string)
                m = messages.Message()
                print(m.ParseFromString(string))
                print(m.SerializeToString())


        except Exception as error:
            print(f"error: {error}")

        time.sleep(3)


asyncio.run(hello())
