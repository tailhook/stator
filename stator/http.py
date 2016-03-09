from  ctypes import c_uint32, c_uint16, c_ssize_t, c_char_p
from  ctypes import c_size_t, c_uint64, c_int64, c_double

from .lib import dll
from .util import convert_ip

dll.stator_http_bind_ipv4.argtypes = [c_uint32, c_uint16]
dll.stator_http_bind_ipv4.restype = c_ssize_t


class CantBindAddress(Exception):
    pass


class Http(object):

    def __init__(self, host, port):
        ip = convert_ip(host)
        self._id = dll.stator_carbon_connect_ipv4(ip, port)
        if self._id <= 0:
            raise CantBindAddress()
