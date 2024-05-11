import random

##
## Settings for the communication
##
import sys

# None, "rsa", "kyber", "frodo", "lightsaber", "firesaber", "mceliece", "ntruhps", "saber", "ntruhrss"
ENCRYPTION_METHOD = "rsa"
#PORT = random.randint(1024, 49151)
PORT = 2522
PORT = int(sys.argv[1]) if len(sys.argv) > 1 and sys.argv[1].isnumeric() else 2522
BUFFER_SIZE = 32768
# "tcp", "udp"
TRANSPORT_LAYER = "tcp"
