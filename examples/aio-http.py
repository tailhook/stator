#!/usr/bin/env python3
"""This is a similar example using aiohttp to benchmark and compare"""

import aiohttp
import aiohttp.server

import asyncio
from urllib.parse import urlparse, parse_qsl
from aiohttp.multidict import MultiDict


class HttpRequestHandler(aiohttp.server.ServerHttpProtocol):

    @asyncio.coroutine
    def handle_request(self, message, payload):
        response = aiohttp.Response(
            self.writer, 200, http_version=message.version)
        bcontent = b"Hello world"
        response.add_header('Content-Type', 'text/html; charset=UTF-8')
        response.add_header('Content-Length', str(len(bcontent)))
        response.add_header('Connection', "close")
        response.send_headers()
        response.write(bcontent)
        yield from response.write_eof()


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    f = loop.create_server(HttpRequestHandler, '0.0.0.0', '3000')
    srv = loop.run_until_complete(f)
    print('serving on', srv.sockets[0].getsockname())
    try:
        loop.run_forever()
    except KeyboardInterrupt:
        pass
