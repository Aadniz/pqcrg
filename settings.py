import random

##
## Settings for the communication
##

# None, "rsa", "kyber", "frodo", "lightsaber", "firesaber", "mceliece", "ntruhps", "saber", "ntruhrss"
ENCRYPTION_METHOD = "ntruhrss"
#PORT = random.randint(1024, 49151)
PORT = 2522
BUFFER_SIZE = 1024
# "tcp", "udp"
TRANSPORT_LAYER = "tcp"
