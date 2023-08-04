from flask import Flask, render_template
from flask_socketio import SocketIO, emit
import time
import threading

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

timeLeft = 30
halt = False
isFinished = False
remainingLaps = 0

@socketio.on('change')
def on_change(value):
  global timeLeft
  timeLeft = max(0, timeLeft + value)

  ping_clients()

@socketio.on('end')
def on_end():
  global timeLeft
  timeLeft = 0

  ping_clients()

@socketio.on('halt')
def on_end():
  global halt
  halt = True

@socketio.on('unHalt')
def on_end():
  global halt
  halt = False

def ping_clients():
  global timeLeft
  global isFinished
  global remainingLaps
  data = {
    'timeLeft': timeLeft,
    'remainingLaps': 0
  }
  # JSON data
  # Ping connected clients every second
  socketio.emit('ping', data, namespace='/')
  time.sleep(1)
  timeLeft = max(0, timeLeft - 1)
  if timeLeft == 0:
    print('Time is up!')
    

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


# External functions
def unHalt():
  global halt
  halt = False

def resetTime():
  global timeLeft
  timeLeft = 30

def setRemainingLaps(laps):
  global remainingLaps
  remainingLaps = laps
  
def on_finish():
  global isFinished
  if isFinished:
    return
  else:
    # Call axel stuff

    isFinished = True

if __name__ == '__main__':
  threading.Thread(target=ping_loop).start()
  socketio.run(app, host='127.0.0.1', port=5800)