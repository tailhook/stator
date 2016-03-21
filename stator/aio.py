import asyncio

from .loop import parse_message, blocking_io
from .lib import dll


def dispatch_messages(loop, callback):
    while True:
        message = dll.stator_next_message(parse_message, blocking_io)
        if message is False:
            break
        elif message is None:
            continue
        elif isinstance(message, BaseException):
            raise message
        else:
            loop.create_task(callback(loop, message))


def start(callback, *, loop=None):
    if loop is None:
        loop = asyncio.get_event_loop()
    assert asyncio.iscoroutinefunction(callback), callback
    loop.add_reader(dll.stator_get_input_fd(),
        dispatch_messages, loop, callback)
