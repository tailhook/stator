# This is an pure-python alternative to stator's carbon implementation
import os
import random
from time import sleep, time, clock
from graphiti.client import Client


def main():
    carbon = Client(os.environ['CARBON_HOST'])
    while True:
        v1 = random.randrange(10, 100)
        v2 = random.randrange(100, 1000)/10.0
        start = time()
        carbon.send("py.stator.random.int", v1)
        carbon.send("py.stator.random.float", v2)
        end = time()
        print(format(end - start, '0.5f'))
        sleep(1)


if __name__ == '__main__':
    main()
