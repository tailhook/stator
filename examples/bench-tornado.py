#!/usr/bin/env python3
"""This is a similar example using tornado to benchmark and compare"""

import tornado.ioloop
import tornado.web

class MainHandler(tornado.web.RequestHandler):
    def get(self):
        self.write("Hello world")

def make_app():
    return tornado.web.Application([
        (r"/", MainHandler),
    ])

if __name__ == "__main__":
    app = make_app()
    app.listen(3000)
    tornado.ioloop.IOLoop.current().start()
