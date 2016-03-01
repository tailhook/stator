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
        start = clock()
        carbon.send("py.stator.random.int", v1)
        carbon.send("py.stator.random.float", v2)
        sleep(60)
        end = clock()
        print(format(end - start, '0.5f'))


if __name__ == '__main__':
    main()
