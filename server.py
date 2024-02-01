import socket
import threading
from encrypt_methods.rsa_encryption import RSAEncryption


def handle_client(connection, client_address, encryption: RSAEncryption):
    host = client_address[0]
    port = client_address[1]
    try:
        # Receive the client's public key
        client_pubkey = connection.recv(1024).decode('utf-8')
        encryption.add_connection(host, client_pubkey)

        # Send the server's public key
        connection.sendall(encryption.handshake().encode('utf-8'))

        # Receive and decrypt the message
        crypto = connection.recv(1024)
        message = encryption.decrypt(crypto).decode('utf-8')
        print(f"[THREAD 1]: received: {message} from {host}:{port}")
    finally:
        connection.close()


def server(encryption_type: str, port: int):
    """
    This function is the listener thread, e.g. the server.
    :return:
    """
    encryption = RSAEncryption()

    # Setting up the socket
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_address = ('127.0.0.1', port)
    sock.bind(server_address)
    sock.listen(1)

    while True:
        print("[THREAD 1]: Waiting for a connection...")
        connection, client_address = sock.accept()

        # Start a new thread to handle this client
        threading.Thread(target=handle_client, args=(connection, client_address, encryption)).start()