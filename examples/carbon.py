import random
from time import sleep, time
from stator import carbon
from time import clock


def main():
    carbon.init_env()
    while True:
        v1 = random.randrange(10, 100)
        v2 = random.randrange(100, 1000)/10.0
        start = time()
        carbon.add("py.stator.random.int", v1)
        carbon.add("py.stator.random.float", v2)
        end = time()
        print(format(end - start, '0.5f'))
        sleep(1)


if __name__ == '__main__':
    main()
