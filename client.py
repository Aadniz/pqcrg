import time
import socket
import struct

from communication_methods.kyber_aes_cbc_encryption import KyberAESCBCEncryption
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

    if ENCRYPTION_METHOD == "rsa":
        encryption = RSAEncryption()
    elif ENCRYPTION_METHOD == "kyber":
        encryption = KyberAESCBCEncryption()
    else:
        exit(1)
    server_address = ('127.0.0.1', PORT)
    encryption.send(server_address, 'Hello from client')
    encryption.send(server_address, 'This is very cool')
    encryption.send(server_address, 'innit!!')
    encryption.send(server_address, 'One')
    encryption.send(server_address, 'Two')
    encryption.send(server_address, 'Three')
    encryption.send(server_address, 'Four')
    encryption.send(server_address, 'YAY!')
    exit(0)