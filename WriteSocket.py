import socket
import time

HOST = "127.0.0.1"  # Standard loopback interface address (localhost)
PORT = 64623  # Port to listen on (non-privileged ports are > 1023)

laps = 0

server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server_socket.bind((HOST, PORT))
server_socket.listen(1)

conn, addr = server_socket.accept()

for i in range(20, 0, -1):
    conn.sendall(bytes(f'$F,{str(i)},"00:00:00","16:34:08","00:00:00","      "\r\n', encoding='utf8'))
    time.sleep(34)
