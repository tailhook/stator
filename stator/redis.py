from ctypes import c_uint64, c_uint32, c_uint16, c_size_t, c_char_p
from ctypes import POINTER, Structure

from .lib import dll
from .util import convert_ip


class Item(Structure):
    _fields_ = [
        ('data', c_char_p),
        ('len', c_size_t),
    ]


dll.stator_redis_connect_ipv4.argtypes = [c_uint32, c_uint16, c_uint32]
dll.stator_redis_connect_ipv4.restype = c_uint64

dll.stator_redis_queue.argtypes = [c_uint64, POINTER(Item), c_size_t]
dll.stator_redis_queue.restype = c_uint64


class CantCreateConnection(Exception):
    pass


class Redis(object):

    def __init__(self, host='127.0.0.1', port=6379, db=0):
        ip = convert_ip(host)
        self._id = dll.stator_redis_connect_ipv4(ip, port, db)
        if self._id <= 0:
            raise CantCreateConnection()

    def command(self, args):
        bargs = list(map(bytes, args))
        num = len(bargs)
        buf = (Item * num)()
        for i, val in enumerate(bargs):
            buf[i].data = val
            buf[i].len = len(val)
        cmd_id = stator_redis_queue(self._id, buf, num)
        print("Redis command id", cmd_id)
