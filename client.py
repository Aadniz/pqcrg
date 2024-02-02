import time
import socket
import struct
from encrypt_methods.rsa_encryption import RSAEncryption

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

    # Setting up the socket
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
    server_address = ('127.0.0.1', PORT)

    # Waiting for server to become available
    while True:
        try:
            sock.connect(server_address)
            break
        except ConnectionRefusedError:
            print("[CLIENT]: Waiting for connection...")
            time.sleep(5)

    # Depending on the encryption schema, this handshake value might vary
    sock.sendall(encryption.handshake().encode('utf-8'))

    # Receive and store the server's public key
    server_pubkey = sock.recv(BUFFER_SIZE).decode('utf-8')
    host, port = sock.getsockname()
    encryption.add_connection(host, server_pubkey)

    # Now that that is done, we can start sending encrypted messages
    send_message(encryption, host, sock, 'Hello from client')
    send_message(encryption, host, sock, 'This is very cool')
    send_message(encryption, host, sock, 'innit!!')
    send_message(encryption, host, sock, 'One')
    send_message(encryption, host, sock, 'Two')
    send_message(encryption, host, sock, 'Three')
    send_message(encryption, host, sock, 'Four')
    send_message(encryption, host, sock, 'YAY!')