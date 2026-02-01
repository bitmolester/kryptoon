# IMPORT
from kryptoon import __internal__ as _internal # type: ignore
import os

# MAIN
class ChaCha20Poly1305:
    def __init__(self, secretkey: bytes) -> None:
        self.secretkey = secretkey
        #
        return
    #
    def encrypt(self, buffer: bytes, *, nonce: bytes | None = None, append: bool = True) -> bytes:
        nonce = nonce if nonce else os.urandom(12)
        ciphertext = _internal.chencrypt(self.secretkey, nonce, buffer) #type: ignore
        if append:
            return nonce + ciphertext # type: ignore
        else:
            return ciphertext # type: ignore
    #
    def decrypt(self, buffer: bytes, *, nonce: bytes | None = None) -> bytes:
        nonce = nonce if nonce else buffer[:12]
        buffer = buffer if nonce else buffer[12:]
        cleartext = _internal.chdecrypt(self.secretkey, nonce, buffer) #type: ignore
        return cleartext #type: ignore
    #
    @staticmethod
    def staticencrypt(secretkey: bytes, buffer: bytes, *, nonce: bytes | None = None, append: bool = True) -> bytes:
        nonce = nonce if nonce else os.urandom(12)
        ciphertext = _internal.chencrypt(secretkey, nonce, buffer) #type: ignore
        if append:
            return nonce + ciphertext # type: ignore
        else:
            return ciphertext # type: ignore
    #
    @staticmethod
    def staticdecrypt(secretkey: bytes, buffer: bytes, *, nonce: bytes | None = None, append: bool = True) -> bytes:
        nonce = nonce if nonce else buffer[:12]
        buffer = buffer if nonce else buffer[12:]
        cleartext = _internal.chdecrypt(self.secretkey, nonce, buffer) #type: ignore
        return cleartext #type: ignore

class XChaCha20Poly1305:
    def __init__(self, secretkey: bytes) -> None:
        self.secretkey = secretkey
        #
        return
    #
    def encrypt(self, buffer: bytes, *, nonce: bytes | None = None, append: bool = True) -> bytes:
        nonce = nonce if nonce else os.urandom(24)
        ciphertext = _internal.xchencrypt(self.secretkey, nonce, buffer) #type: ignore
        if append:
            return nonce + ciphertext # type: ignore
        else:
            return ciphertext # type: ignore
    #
    def decrypt(self, buffer: bytes, *, nonce: bytes | None = None) -> bytes:
        nonce = nonce if nonce else buffer[:24]
        buffer = buffer if nonce else buffer[24:]
        cleartext = _internal.xchdecrypt(self.secretkey, nonce, buffer) #type: ignore
        return cleartext #type: ignore
    #
    @staticmethod
    def staticencrypt(secretkey: bytes, buffer: bytes, *, nonce: bytes | None = None, append: bool = True) -> bytes:
        nonce = nonce if nonce else os.urandom(24)
        ciphertext = _internal.xchencrypt(secretkey, nonce, buffer) #type: ignore
        if append:
            return nonce + ciphertext # type: ignore
        else:
            return ciphertext # type: ignore
    #
    @staticmethod
    def staticdecrypt(secretkey: bytes, buffer: bytes, *, nonce: bytes | None = None, append: bool = True) -> bytes:
        nonce = nonce if nonce else buffer[:24]
        buffer = buffer if nonce else buffer[24:]
        cleartext = _internal.xchdecrypt(self.secretkey, nonce, buffer) #type: ignore
        return cleartext #type: ignore
