#!/usr/bin/env python
# Copyright (c) 2021 Andrew Paxson. All rights reserved. Used under
# Licensed under the MIT License. See LICENSE file in the project root for full license information.
""" client.py """
import time
import websocket

import test_client.messages_pb2 as messages

class ExampleClass(object):
    def __init__(self):
        websocket.enableTrace(True)
        self.ws = websocket.WebSocketApp("ws://localhost:8080",
                                         on_open=self.on_open,
                                         on_message=self.on_message,
                                         on_error=self.on_error,
                                         on_close=self.on_close)

    def on_message(self, ws, msg):
        print("on_message")
        print(msg)
        m = messages.Message()
        print(m.ParseFromString(msg))
        print(m.SerializeToString())

    def on_error(self, ws, error):
        print(error)

    def on_close(self, ws, close_status_code, close_msg):
        print("Connection Closed")

    def on_open(self, ws):
        print("connected....sending...")
        m = messages.Message()
        m.session_start.session_properties.api_version = "1.0"
        m.session_start.session_properties.session_id = "mysession"
        ws.send(m.SerializeToString(), websocket.ABNF.OPCODE_BINARY)
        print("sent.")

    def run(self):
        self.ws.run_forever()

def main():
    ws = ExampleClass()
    while True:
        ws.run()
        time.sleep(1)

main()