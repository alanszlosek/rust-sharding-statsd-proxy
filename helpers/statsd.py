import os
import socket

PROXY_IP = "127.0.0.1" # "192.168.1.120"
PROXY_PORT = os.environ['PORT'] if 'PORT' in os.environ else 8125

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
f = open("../messages.txt", 'r')
# precompute:. convert lines to byte arrays for speed
lines = [line.encode('utf8') for line in f.readlines()]

while 1:
    for line in lines:
        sock.sendto(line, (PROXY_IP, PROXY_PORT))
