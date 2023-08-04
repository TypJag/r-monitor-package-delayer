import socket
import time

current_lap = 0

timer_length = 30



def start_timer():
    print("Starting timer")
    endtime = time.time() + timer_length    


    


HOST = "127.0.0.1"  # The server's hostname or IP address
PORT = 64623  # The port used by the server
PORT2 = 64624

Orbits_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
Orbits_socket.connect((HOST, PORT))

Pixel_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
Pixel_socket.bind((HOST, PORT2))
Pixel_socket.listen(1)

conn, addr = Pixel_socket.accept()

while True:
    data = Orbits_socket.recv(1024)

    sdata = data.decode()
    sdata = sdata.split(",")
    #print(sdata)

    #send data to pixel
    conn.sendall(data)


        #if sdata[0] == '$F':
        #    if sdata[1] > current_lap:
        #        start_timer()







            

        



