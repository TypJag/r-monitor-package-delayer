from flask import Flask, render_template
from flask_socketio import SocketIO, emit
import time
import threading
import socket
from functions import checkHasLeaderPassedAndLaps
from functions import sendToPixel 



app = Flask(__name__)
app.config['SECRET_KEY'] = 'your_secret_key'
socketio = SocketIO(app)

@app.route('/')
def index():
  return render_template('index.html')

@socketio.on('connect')
def on_connect():
  print('Client connected')
  # Send a message to the client upon successful connection
  emit('message', {'data': 'Connected to server'})

defaultTime = 30
timeLeft = defaultTime
halt = False
isFinished = False
remainingLaps = 0

HOST = "192.168.10.102"  # The server's hostname or IP address
sendHOST = "192.168.10.108"
recvPORT = 50000  # The port used by the server
sendPORT = 50001

Pixel_conn = 0

@socketio.on('change')
def on_change(value):
  global timeLeft
  timeLeft = max(0, timeLeft + value)

  ping_clients()

@socketio.on('defaultChange')
def on_defaultChange(value):
  global defaultTime
  defaultTime = defaultTime + value

  ping_clients()

@socketio.on('end')
def on_end():
  global timeLeft
  global isFinished
  isFinished = False
  timeLeft = 0

  ping_clients()

@socketio.on('halt')
def on_end():
  global halt
  halt = True

  ping_clients()

@socketio.on('unHalt')
def on_end():
  global halt
  halt = False

  ping_clients()

@socketio.on('reset')
def on_end():
  global timeLeft
  global isFinished
  timeLeft = defaultTime
  isFinished = False

  ping_clients()

def ping_clients():
  global timeLeft
  global isFinished
  global remainingLaps
  data = {
    'timeLeft': timeLeft,
    'remainingLaps': remainingLaps,
    'defaultTime': defaultTime,
  }
  # JSON data
  # Ping connected clients every second
  socketio.emit('ping', data, namespace='/')

    

def ping_loop():
  global timeLeft
  global halt
  while True:
    if halt:
      print('Halted')
      time.sleep(1)
    else:
      print('not halted')
      ping_clients()
      time.sleep(1)
      timeLeft = max(0, timeLeft - 1)
      if timeLeft == 0:
        #print('Time is up!')
        on_finish()


# External functions
def unHalt():
  global halt
  halt = False

def resetTime():
  global isFinished
  isFinished = False
  global timeLeft
  timeLeft = defaultTime

def setRemainingLaps(laps):
  global remainingLaps
  remainingLaps = laps
  
def on_finish():
  global isFinished
  if isFinished:
    return
  else:
    # Call axel stuff
    sendToPixel(Pixel_conn,remainingLaps)
    isFinished = True

def tcp_loop():
  global HOST
  global recvPORT
  global sendPORT
  global remainingLaps
  global Pixel_conn

  Orbits_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
  Orbits_socket.connect((HOST, recvPORT))

  Pixel_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
  Pixel_socket.bind((sendHOST, sendPORT))
  Pixel_socket.listen(1)

  Pixel_conn, addr = Pixel_socket.accept()
  print("Connected to Pixel")

  while True:
    [hasPassed, lap] = checkHasLeaderPassedAndLaps(Orbits_socket,remainingLaps)
    
    if (hasPassed == True):
      print("Leader has passed")
      resetTime()
      setRemainingLaps(lap)
  

if __name__ == '__main__':
  threading.Thread(target=ping_loop).start()
  threading.Thread(target=tcp_loop).start()
  socketio.run(app, host='127.0.0.1', port=5800)