import random
from time import sleep, time
from stator import http
from time import clock


def main():
    sock = http.Http('127.0.0.1', 3000)
    print(sock.__dict__)


if __name__ == '__main__':
    main()
