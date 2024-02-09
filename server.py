import socket
import struct
from communication_methods.rsa_encryption import RSAEncryption
from communication_methods.kyber_aes_cbc_encryption import KyberAESCBCEncryption

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
    if ENCRYPTION_METHOD == "rsa":
        encryption = RSAEncryption()
    elif ENCRYPTION_METHOD == "kyber":
        encryption = KyberAESCBCEncryption()
    else:
        exit(1)

    encryption.listen()
