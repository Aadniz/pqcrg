import socket
import struct
from encrypt_methods.rsa_encryption import RSAEncryption

from settings import PORT
from settings import BUFFER_SIZE
from settings import ENCRYPTION_METHOD

def recv_message(encryption, connection) -> str|bool:
    # Receive the length of the message (4 bytes)
    first_four_bytes = connection.recv(4)
    if not first_four_bytes:
        return False
    length = struct.unpack('!I', first_four_bytes)[0]

    # Receive the rest of the message
    crypto = connection.recv(length)

    # Decrypt the message
    return encryption.decrypt(crypto).decode('utf-8')

def server():
    """
    This function is the listener thread, e.g. the server.
    :return:
    """
    encryption = RSAEncryption()

    # Setting up the socket
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_address = ('127.0.0.1', PORT)
    sock.bind(server_address)
    sock.listen(1)

    while True:
        print("[SERVER]: Waiting for a connection...")
        connection, client_address = sock.accept()

        host = client_address[0]
        port = client_address[1]

        if not encryption.has_connection(host):
            # Receive the client's public key
            client_pubkey = connection.recv(BUFFER_SIZE).decode('utf-8')
            encryption.add_connection(host, client_pubkey)

            # Send the server's public key
            connection.sendall(encryption.handshake().encode('utf-8'))

        while True:
            message = recv_message(encryption, connection)
            if not message:
                break
            print(f"[SERVER]: received: {message} from {host}:{port}")