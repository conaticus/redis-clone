import socket

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect(("127.0.0.1", 6379))
sock.sendall("*2\r\n$4\r\nECHO\r\n$3\r\nhey\r\n".encode())
data = sock.recv(1024)
print(data.decode())
