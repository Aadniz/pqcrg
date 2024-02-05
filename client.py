import time
import socket
import struct
from communication_methods.rsa_encryption import RSAEncryption

from settings import PORT
from settings import BUFFER_SIZE
from settings import ENCRYPTION_METHOD


def send_message(encryption, host, sock, message):
    print(f"[CLIENT]: Sending message: {message} ...")
    # Encrypt the message
    crypto = encryption.encrypt(host, message)

    # Pack the length of the message as a 4-byte integer
    length = struct.pack('!I', len(crypto))

    # Send the length followed by the message
    sock.sendall(length + crypto)


def client():
    """
    This function is the sender thread, eg. the client.
    :return:
    """

    encryption = RSAEncryption()
    server_address = ('127.0.0.1', PORT)
    encryption.send(server_address, 'Hello from client')
    encryption.send(server_address, 'This is very cool')
    encryption.send(server_address, 'innit!!')
    encryption.send(server_address, 'One')
    encryption.send(server_address, 'Two')
    encryption.send(server_address, 'Three')
    encryption.send(server_address, 'Four')
    encryption.send(server_address, 'YAY!')
    encryption.close(server_address)