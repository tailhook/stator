from io import BytesIO
from ctypes import CFUNCTYPE, POINTER, c_uint64, c_char, c_size_t, c_uint8
from ctypes import py_object, c_void_p

from .lib import dll


_message_parser = CFUNCTYPE(py_object, c_uint64, c_void_p, c_size_t)
dll.stator_wait_message.argtypes = [_message_parser]
dll.stator_wait_message.restype = py_object


class Socket(object):

    def __init__(self, id):
        assert isinstance(id, (int, long)), (type(id), id)
        self.id = id
        table.add(self)

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


table = SocketTable()

@_message_parser
def _parse_message(sock_id, buf, buf_len):
    sock = table.get(sock_id)
    if sock is None:
        return None
    buf = BytesIO((c_uint8 * buf_len).from_address(buf))
    return sock.parse_message(buf)

def events():
    while True:
        message = dll.stator_wait_message(_parse_message)
        if message:
            yield message
