import socket

def checkHasLeaderPassed(host_socket, currentlap):
    
    while True:
        data = host_socket.recv(1024)
        sdata = data.decode()
        sdata = sdata.split(",")
        if sdata[0] == '$F':
            if sdata[1] != currentlap:
                return True
            else:
                return False
        elif sdata[0] == '':
            return    

def checkRemaingingLaps(host_socket, currentlap):
    while True:
        data = host_socket.recv(1024)
        sdata = data.decode()
        sdata = sdata.split(",")
        if sdata[0] == '$F':
            if sdata[1] != currentlap:
                return sdata[1]
            
def sendToPixel(conn, remainingLaps):
    for i in range(5):
        conn.sendall(bytes(f'$F,{remainingLaps},"00:00:00","00:00:00","00:00:00","      "\r\n', encoding='utf8'))

    
        