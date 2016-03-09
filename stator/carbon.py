import os
from  ctypes import c_uint32, c_uint16, c_ssize_t, c_char_p
from  ctypes import c_size_t, c_uint64, c_int64, c_double

from .lib import dll
from .util import convert_ip

dll.stator_carbon_connect_ipv4.argtypes = [c_uint32, c_uint16]
dll.stator_carbon_connect_ipv4.restype = c_ssize_t

dll.stator_carbon_add_i64.argtypes = [c_ssize_t, c_char_p, c_size_t, c_int64]
dll.stator_carbon_add_i64.restype = None
dll.stator_carbon_add_i64_at.argtypes = [c_ssize_t, c_char_p, c_size_t,
                                  c_int64, c_uint64]
dll.stator_carbon_add_i64_at.restype = None

dll.stator_carbon_add_f64.argtypes = [c_ssize_t, c_char_p, c_size_t, c_double]
dll.stator_carbon_add_f64.restype = None
dll.stator_carbon_add_f64_at.argtypes = [c_ssize_t, c_char_p, c_size_t,
                                  c_double, c_uint64]
dll.stator_carbon_add_f64_at.restype = None


default_instance = None


class CantCreateConnection(Exception):
    pass


class Carbon(object):
    """The wrapper around carbon connection in Rust code"""

    def __init__(self, host, port=2003):
        """Create a connection

        :param host: **ip address** of the carbon server
        :param port: port of the carbon server
        """
        ip = convert_ip(host)
        self._id = dll.stator_carbon_connect_ipv4(ip, port)
        if self._id <= 0:
            raise CantCreateConnection()

    def add(self, key, value, timestamp=None):
        """Add a value to carbon stats

        :param:`key` a full metric name
        :param:`value` a value. Might be integer that must fit signed 64bit
            integer, or a floating point value
        :param:`timestamp` a timestamp of the data point.
            Default is ``time.time()``.
        """
        key = key.encode('utf-8')
        key_len = len(key)
        if isinstance(value, int):
            if timestamp is None:
                dll.stator_carbon_add_i64(self._id, key, key_len, value)
            else:
                dll.stator_carbon_add_i64_at(self._id,
                    key, key_len, value, timestamp)
        else:
            if timestamp is None:
                dll.stator_carbon_add_f64(self._id, key, key_len, value)
            else:
                dll.stator_carbon_add_f64_at(self._id,
                    key, key_len, value, timestamp)


def init_env():
    """Initialize a default instance and instance from environment variables

    Environment variables ``CARBON_HOST``, ``CARBON_PORT``. The latter is
    optional.

    If no ``CARBON_HOST`` variable present or ``default_instance`` in this
    module is not initialized, function ``add`` in this module is just no-op.
    In other words, statistics are enabled by setting environment variable
    """
    global default_instance
    host = os.environ.get("CARBON_HOST")
    if host:
        default_instance = Carbon(host,
            int(os.environ.get("CARBON_PORT", 2003)))


def add(key, value, timestamp=None):
    """Add a value to default instance of Carbon

    :param key: a full metric name
    :param value: a value. Might be integer that must fit signed 64bit
        integer, or a floating point value
    :param timestamp: a timestamp of the data point.
        Default is ``time.time()``.
    """

    if default_instance:
        default_instance.add(key, value, timestamp)
