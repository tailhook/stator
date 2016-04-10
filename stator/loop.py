import sys
import logging
from io import BytesIO
from ctypes import CFUNCTYPE, POINTER, c_uint64, c_char, c_size_t, c_uint8
from ctypes import py_object, c_void_p, c_int

from .lib import dll


log = logging.getLogger(__name__)
_message_parser = CFUNCTYPE(py_object, c_uint64, c_void_p, c_size_t)
_skip = CFUNCTYPE(py_object)
dll.stator_wait_message.argtypes = [_message_parser, _skip]
dll.stator_wait_message.restype = py_object
dll.stator_next_message.argtypes = [_message_parser, _skip]
dll.stator_next_message.restype = py_object
dll.stator_get_input_fd.argtypes = []
dll.stator_get_input_fd.restype = c_int

if sys.version_info < (3, 0):
    INT_TYPES = (int, long)
else:
    INT_TYPES = int


class Socket(object):

    def __init__(self, id):
        assert isinstance(id, INT_TYPES), (type(id), id)
        self.id = id
        TABLE.add(self)

    def close(self):
        TABLE.remove(self.id)

    def parse_message(self, input):
        raise RuntimeError("abstract method")

    def __repr__(self):
        return '<{} {}>'.format(self.__class__.__name__,
            repr(self.__dict__)[1:-1])


class SocketTable(object):

    def __init__(self):
        self._sockets = {}

    def add(self, item):
        self._sockets[item.id] = item

    def get(self, id):
        return self._sockets.get(id)

    def remove(self, id):
        self._sockets.pop(id, None)


TABLE = SocketTable()

@_message_parser
def parse_message(sock_id, buf, buf_len):
    try:
        sock = TABLE.get(sock_id)
        if sock is None:
            log.debug("Got message for socket {} which is already closed",
                sock_id)
            return None
        buf = BytesIO((c_uint8 * buf_len).from_address(buf))
        return sock.parse_message(buf)
    except BaseException as e:
        return e


@_skip
def no_message():
    return None

@_skip
def blocking_io():
    return False


def events():
    while True:
        message = dll.stator_wait_message(parse_message, no_message)
        if message is not None:
            if isinstance(message, BaseException):
                raise message
            else:
                yield message
