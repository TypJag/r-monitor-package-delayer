import socket
import time

def checkHasLeaderPassedAndLaps(host_socket, currentlap):
    
    while True:
        data = host_socket.recv(1024)
        sdata = data.decode()
        sdata = sdata.split(",")
        if sdata[0] == '$F':
            if sdata[1] != currentlap:
                return True, sdata[1]
            else:
                return False, currentlap
        elif sdata[0] == '':
            return    

def checkRemaingingLaps(host_socket, currentlap):
    while True:
        data = host_socket.recv(1024)
        sdata = data.decode()
        sdata = sdata.split(",")
        time.sleep(0.1)
        if sdata[0] == '$F':
            if sdata[1] != currentlap:
                return sdata[1]
            
def sendToPixel(conn, remainingLaps):
    print("Sending to pixel")
    for i in range(5):
        conn.sendall(bytes(f'$F,{remainingLaps},"00:00:00","00:00:00","00:00:00","      "\r\n', encoding='utf8'))

    
        