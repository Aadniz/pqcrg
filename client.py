import time
import socket
from encrypt_methods.rsa_encryption import RSAEncryption

def client(encryption_type: str, port: int):
    """
    This function is the sender thread, eg. the client.
    :return:
    """
    time.sleep(5)
    encryption = RSAEncryption()

    # Setting up the socket
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_address = ('127.0.0.1', port)

    # Waiting for server to become available
    while True:
        try:
            sock.connect(server_address)
            break
        except ConnectionRefusedError:
            print("[THREAD 2]: Waiting for connection...")
            time.sleep(5)

    # Depending on the encryption schema, this handshake value might vary
    sock.sendall(encryption.handshake().encode('utf-8'))

    # Receive and store the server's public key
    server_pubkey = sock.recv(1024).decode('utf-8')
    host, port = sock.getsockname()
    encryption.add_connection(host, server_pubkey)

    # Now that that is done, we can start sending encrypted messages
    message = 'Hello from Thread 2'
    crypto = encryption.encrypt(host, message)
    sock.sendall(crypto)
