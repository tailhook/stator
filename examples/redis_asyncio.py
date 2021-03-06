import asyncio
from stator import http, redis, loop, aio


def main():
    loop = asyncio.get_event_loop()
    red = aio.Redis(port=3001)

    @asyncio.coroutine
    def dispatch(loop, req):
        n = yield from red.execute([b"INCR", b"hello-world-counter"])
        req.reply(
            [200, u"OK"],
            {u"Content-Type": b"text/html"},
            "Hello page opened {} times".format(n).encode('utf-8'))

    sock = http.Http('0.0.0.0', 3000)
    aio.start(dispatch, loop=loop)

    print("Listening on http://0.0.0.0:3000/")
    loop.run_forever()


if __name__ == '__main__':
    main()
