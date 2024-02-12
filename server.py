from communication_methods.rsa_encryption import RSAEncryption
from communication_methods.kyber_aes_cbc_encryption import KyberAESCBCEncryption
from communication_methods.plain import Plain

from settings import PORT
from settings import BUFFER_SIZE
from settings import ENCRYPTION_METHOD
from settings import TRANSPORT_LAYER


def server():
    """
    This function is the listener thread, e.g. the server.
    :return:
    """
    if ENCRYPTION_METHOD == "rsa":
        encryption = RSAEncryption(TRANSPORT_LAYER)
    elif ENCRYPTION_METHOD == "kyber":
        encryption = KyberAESCBCEncryption(TRANSPORT_LAYER)
    elif ENCRYPTION_METHOD is None:
        encryption = Plain(TRANSPORT_LAYER)
    else:
        raise Exception(f"{ENCRYPTION_METHOD} not supported")

    encryption.listen()
