import random
from time import sleep, time
from stator import http, loop
from time import clock


def main():
    sock = http.Http('127.0.0.1', 3000)
    for req in loop.events():
        req.reply(
            [200, u"OK"],
            {u"Content-Type": b"text/html"},
            b"Hello World")


if __name__ == '__main__':
    main()
