import random
from time import sleep
from stator import carbon


def main():
    carbon.init_env()
    while True:
        v1 = random.randrange(10, 100)
        v2 = random.randrange(100, 1000)/10.0
        carbon.add("py.stator.random.int", v1)
        carbon.add("py.stator.random.float", v2)
        sleep(1)


if __name__ == '__main__':
    main()
