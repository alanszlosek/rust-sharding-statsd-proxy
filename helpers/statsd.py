import os
import socket
import time

PROXY_IP = "127.0.0.1" # "192.168.1.120"
PROXY_PORT = 8125

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
while 1:
    # LOGINS
    # variation 1
    s = 'users.logins,host=%s,country=US:1|c' % (socket.gethostname(),)
    sock.sendto(s.encode('utf8'), (PROXY_IP, PROXY_PORT))
    # variation 2 - different tag order
    s = 'users.logins,country=US,host=%s:1|c' % (socket.gethostname(),)
    sock.sendto(s.encode('utf8'), (PROXY_IP, PROXY_PORT))

    # PURCHASE
    s = 'users.purchase,country=US,host=%s:1|c' % (socket.gethostname(),)
    sock.sendto(s.encode('utf8'), (PROXY_IP, PROXY_PORT))

    # ADD TO CART
    s = 'users.add_to_cart,country=US,host=%s:1|c' % (socket.gethostname(),)
    sock.sendto(s.encode('utf8'), (PROXY_IP, PROXY_PORT))

    #time.sleep(1)
