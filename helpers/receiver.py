import os
import socket
import time

PORT = int(os.environ['PORT']) if 'PORT' in os.environ else 5002
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.bind(("0.0.0.0", PORT))

i = 0
while True:
    data, addr = sock.recvfrom(1024) # buffer size is 1024 bytes
    i += 1

    if i > 1000:
        s = 'sharding_proxy.metrics_received,host=%s:%d|c' % ("gigabyte",i)
        sock.sendto(s.encode('utf8'), ('192.168.1.173', 8125))
        i = 0
    #print("Received: ", data)