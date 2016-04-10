import redis


rconn = redis.Redis(unix_socket_path="/work/target/redis.sock")


def application(env, start_response):
    start_response('200 OK', [('Content-Type','text/html')])
    n = rconn.incr("hello-world-counter")
    return ["Hello page opened {} times".format(n).encode('utf-8')]
