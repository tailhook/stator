import asyncio
import cbor
from collections import namedtuple

from functools import partial
from .loop import parse_message, blocking_io, Socket
from .lib import dll
from . import redis


LOOP = None
FUTURES = {}
_RedisResponse = namedtuple("_RedisResponse", ["future_id", "value"])


class Redis(Socket):

    def __init__(self, *, host='127.0.0.1', port=6379, db=0):
        id = redis.connect_ip(host=host, port=port, db=db)
        super(Redis, self).__init__(id)

    def execute(self, args):
        cmd_id = redis.command(self.id, args)
        f = asyncio.Future(loop=LOOP)
        f.add_done_callback(partial(FUTURES.pop, cmd_id))
        FUTURES[cmd_id] = f
        return f

    def parse_message(self, input):
        req_id = cbor.load(input)
        data = cbor.load(input)
        return _RedisResponse(req_id, data)


def dispatch_messages(loop, callback):
    while True:
        message = dll.stator_next_message(parse_message, blocking_io)
        if message is False:
            break
        elif message is None:
            continue
        elif isinstance(message, BaseException):
            raise message
        elif isinstance(message, _RedisResponse):
            # TODO(tailhook) check error
            FUTURES[message.future_id].set_result(message.value)
        else:
            loop.create_task(callback(loop, message))


def start(callback, *, loop=None):
    global LOOP
    if loop is None:
        loop = asyncio.get_event_loop()
    LOOP = loop
    assert asyncio.iscoroutinefunction(callback), callback
    loop.add_reader(dll.stator_get_input_fd(),
        dispatch_messages, loop, callback)
