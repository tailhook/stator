import cbor

from  ctypes import c_uint64, c_uint32, c_uint16, c_ssize_t, c_char_p
from  ctypes import c_size_t, c_uint64, c_int64, c_double, c_void_p

from .lib import dll
from .util import convert_ip
from .loop import Socket

dll.stator_http_bind_ipv4.argtypes = [c_uint32, c_uint16]
dll.stator_http_bind_ipv4.restype = c_uint64

dll.stator_http_reply.argtypes = [c_uint64, c_void_p, c_size_t]
dll.stator_http_reply.restype = None


class CantBindAddress(Exception):
    pass


class HttpRequest(Socket):

    def __init__(self, id, dic):
        super(HttpRequest, self).__init__(id)
        self.__dict__.update(dic)

    def reply(self, status, headers, body):
        assert 200 <= status[0] <= 599, status
        assert isinstance(status[1], unicode), status
        assert isinstance(headers, dict), headers
        assert isinstance(body, bytes), body
        data = cbor.dumps([status, headers, body])
        dll.stator_http_reply(self.id, data, len(data))
        self.close()


class Http(Socket):

    def parse_message(self, input):
        dic = cbor.load(input)
        return HttpRequest(dic['response_socket'], dic)

    def __init__(self, host, port):
        ip = convert_ip(host)
        id = dll.stator_http_bind_ipv4(ip, port)
        super(Http, self).__init__(id)
