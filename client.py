import socket

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect(("127.0.0.1", 6379))
sock.sendall("*3\r\n$3\r\nSET\r\n$5\r\nmykey\r\n$7\r\nmyvalue\r\n".encode())
data = sock.recv(1024)
print(data.decode())
sock.sendall("*3\r\n$3\r\nGET\r\n$5\r\nmykey\r\n".encode())
data = sock.recv(1024)
print(data.decode())
