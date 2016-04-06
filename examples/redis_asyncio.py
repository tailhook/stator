import asyncio
from stator import http, redis, loop, aio


@asyncio.coroutine
def dispatch(loop, req):
    req.reply(
        [200, u"OK"],
        {u"Content-Type": b"text/html"},
        b"Hello World")


def main():
    loop = asyncio.get_event_loop()

    sock = http.Http('0.0.0.0', 3000)
    red = redis.Redis(port=3001)
    aio.start(dispatch, loop=loop)

    loop.run_forever()


if __name__ == '__main__':
    main()
