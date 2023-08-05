import socket
import time

def checkHasLeaderPassedAndLaps(host_socket, currentlap):
    
    while True:
        data = host_socket.recv(1024)
        sdata = data.decode('latin-1')
        sdata = sdata.split(",")
        if sdata[0] == '$F':
            if sdata[1] != currentlap:
                if sdata[1] == '9999':
                    return True, 0
                else:
                    return True, sdata[1]
            else:
                return False, currentlap
        elif sdata[0] == '':
            return    

            
def sendToPixel(conn, remainingLaps):
    print("Sending to pixel")
    for i in range(5):
        tempremLaps = int(remainingLaps) -1
        
        conn.sendall(bytes(f'$F,{str(tempremLaps)},"00:00:00","00:00:00","00:00:00","      "\r\n', encoding='utf8'))

def sendToPixel2(conn, remainingLaps):
    print("Sending to pixel")
    for i in range(5):  
        conn.sendall(bytes(f'$F,{str(remainingLaps)},"00:00:00","00:00:00","00:00:00","      "\r\n', encoding='utf8'))

    
        