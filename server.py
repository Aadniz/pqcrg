from communication_methods.rsa_encryption import RSAEncryption
from communication_methods.kyber_aes_cbc_encryption import KyberAESCBCEncryption
from communication_methods.plain import Plain

from settings import PORT
from settings import BUFFER_SIZE
from settings import ENCRYPTION_METHOD


def server():
    """
    This function is the listener thread, e.g. the server.
    :return:
    """
    if ENCRYPTION_METHOD == "rsa":
        encryption = RSAEncryption()
    elif ENCRYPTION_METHOD == "kyber":
        encryption = KyberAESCBCEncryption()
    elif ENCRYPTION_METHOD == None:
        encryption = Plain()
    else:
        raise Exception(f"{ENCRYPTION_METHOD} not supported")

    encryption.listen()
