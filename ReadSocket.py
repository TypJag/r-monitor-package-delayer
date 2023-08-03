import socket

HOST = "192.168.10.106"  # The server's hostname or IP address
PORT = 50000  # The port used by the server
while True:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((HOST, PORT))
        data = s.recv(1024)

    print(f"Received {data!r}")