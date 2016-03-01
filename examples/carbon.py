import random
from time import sleep
from stator import carbon
from time import clock


def main():
    carbon.init_env()
    while True:
        v1 = random.randrange(10, 100)
        v2 = random.randrange(100, 1000)/10.0
        start = clock()
        carbon.add("py.stator.random.int", v1)
        carbon.add("py.stator.random.float", v2)
        sleep(1)
        end = clock()
        print(format(end - start, '0.5f'))


if __name__ == '__main__':
    main()
