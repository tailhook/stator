import hiredis
import redis
import random
from time import sleep, time
from stator import http, loop
from time import clock


def main():
    sock = http.Http('127.0.0.1', 3000)
    #rconn = redis.Redis(unix_socket_path="/work/target/redis.sock")
    rconn = redis.Redis(port=3001)
    for req in loop.events():
        n = rconn.incr("hello-world-counter")
        req.reply(
            [200, u"OK"],
            {u"Content-Type": b"text/html"},
            "Hello page opened {} times".format(n).encode('utf-8'))


if __name__ == '__main__':
    main()
