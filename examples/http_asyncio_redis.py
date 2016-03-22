import asyncio
import aioredis

from functools import partial

from stator import http, loop, aio




def main():
    loop = asyncio.get_event_loop()
    redis = loop.run_until_complete(
        aioredis.create_redis("/work/target/redis.sock", loop=loop))
    #redis = loop.run_until_complete(
    #    aioredis.create_redis(("127.0.0.1", 3001), loop=loop))

    @asyncio.coroutine
    def dispatch(loop, req):
        n = yield from redis.incr("hello-world-counter")
        req.reply(
            [200, u"OK"],
            {u"Content-Type": b"text/html"},
            "Hello page opened {} times".format(n).encode('utf-8'))

    sock = http.Http('0.0.0.0', 3000)
    aio.start(dispatch, loop=loop)

    loop.run_forever()


if __name__ == '__main__':
    main()
