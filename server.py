from communication_methods.firesaberkem_aes_cbc_encryption import FiresaberKEMAESCBCEncryption
from communication_methods.mceliecekem_aes_cbc_encryption import McelieceKEMAESCBCEncryption
from communication_methods.ntruhpskem_aes_cbc_encryption import NtruhpsKEMAESCBCEncryption
from communication_methods.lightsaberkem_aes_cbc_encryption import LightsaberKEMAESCBCEncryption
from communication_methods.saberkem_aes_cbc_encryption import SaberKEMAESCBCEncryption
from communication_methods.ntruhrsskem_aes_cbc_encryption import NtruhrssKEMAESCBCEncryption
from communication_methods.frodokem_aes_cbc_encryption import FrodoKEMAESCBCEncryption
from communication_methods.kyber_aes_cbc_encryption import KyberAESCBCEncryption
from communication_methods.rsa_encryption import RSAEncryption
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
    elif ENCRYPTION_METHOD == "frodo":
        encryption = FrodoKEMAESCBCEncryption(TRANSPORT_LAYER)
    elif ENCRYPTION_METHOD == "firesaber":
        encryption = FiresaberKEMAESCBCEncryption(TRANSPORT_LAYER)
    elif ENCRYPTION_METHOD == "mceliece":
        encryption = McelieceKEMAESCBCEncryption(TRANSPORT_LAYER)
    elif ENCRYPTION_METHOD == "ntruhps":
        encryption = NtruhpsKEMAESCBCEncryption(TRANSPORT_LAYER)
    elif ENCRYPTION_METHOD == "lightsaber":
        encryption = LightsaberKEMAESCBCEncryption(TRANSPORT_LAYER)
    elif ENCRYPTION_METHOD == "saber":
        encryption = SaberKEMAESCBCEncryption(TRANSPORT_LAYER)
    elif ENCRYPTION_METHOD == "ntruhrss":
        encryption = NtruhrssKEMAESCBCEncryption(TRANSPORT_LAYER)
    elif ENCRYPTION_METHOD is None:
        encryption = Plain(TRANSPORT_LAYER)
    else:
        raise Exception(f"{ENCRYPTION_METHOD} not supported")

    encryption.listen()
