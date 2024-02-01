import rsa
import threading
import queue

# Create a queue for communication
q = queue.Queue()

ENCRYPTION_METHOD = "rsa"
(pubkey, privkey) = rsa.newkeys(512)

def thread1():
    message = 'Hello from Thread 1'.encode('utf8')
    crypto = rsa.encrypt(message, pubkey)
    q.put(crypto)

def thread2():
    crypto = q.get()
    message = rsa.decrypt(crypto, privkey).decode('utf8')
    print(f"Thread 2 received: {message}")

if __name__ == "__main__":
    # Create threads
    t1 = threading.Thread(target=thread1)
    t2 = threading.Thread(target=thread2)

    # Start threads
    t1.start()
    t2.start()

    # Wait for both threads to finish
    t1.join()
    t2.join()
