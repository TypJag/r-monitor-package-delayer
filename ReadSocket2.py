import socket
import time

current_lap = 0

timer_length = 30



def start_timer():
    print("Starting timer")
    endtime = time.time() + timer_length    


    


HOST = "127.0.0.1"  # The server's hostname or IP address
PORT = 64624  # The port used by the server
PORT2 = 64624

Orbits_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
Orbits_socket.connect((HOST, PORT))


while True:
    data = Orbits_socket.recv(1024)

    sdata = data.decode()
    sdata = sdata.split(",")
    print(sdata)