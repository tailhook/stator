import random
from time import sleep, time
from stator import http, loop
from time import clock


def main():
    sock = http.Http('127.0.0.1', 3000)
    for event in loop.events():
        print("EVENT", event)


if __name__ == '__main__':
    main()
