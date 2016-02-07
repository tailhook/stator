# This is an pure-python alternative to stator's carbon implementation
import os
import random
from time import sleep
from graphiti.client import Client


def main():
    carbon = Client(os.environ['CARBON_HOST'])
    while True:
        v1 = random.randrange(10, 100)
        v2 = random.randrange(100, 1000)/10.0
        carbon.send("py.graphiti.random.int", v1)
        carbon.send("py.graphiti.random.float", v2)
        sleep(1)


if __name__ == '__main__':
    main()
