import socket
import time

HOST = "127.0.0.1"  # Standard loopback interface address (localhost)
PORT = 50000  # Port to listen on (non-privileged ports are > 1023)

laps = 0

server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server_socket.bind((HOST, PORT))
server_socket.listen(1)

conn, addr = server_socket.accept()
while True:
    for i in range(1,10):
        conn.sendall(bytes(f'$F,10,"00:00:00","16:34:08","00:00:00","      "\r\n', encoding='utf8'))
        time.sleep(1)

    for i in range(10, 1, -1):
        conn.sendall(bytes(f'$F,{str(i)},"00:00:00","16:34:08","00:00:00","      "\r\n', encoding='utf8'))
        time.sleep(7)

    for i in range (1,20):
        conn.sendall(bytes(f'$F,9999,"00:00:00","16:34:08","00:00:00","      "\r\n', encoding='utf8'))
        time.sleep(5)
    time.sleep(30)

