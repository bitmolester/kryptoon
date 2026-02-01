# MAIN
class ChaCha20Poly1305:
    def __init__(self, secretkey: bytes) -> None:
        ...
    #
    def encrypt(self, buffer: bytes, *, nonce: bytes | None = None, append: bool = True) -> bytes:
        ...
    #
    def decrypt(self, buffer: bytes, *, nonce: bytes | None = None) -> bytes:
        ...
    #
    @staticmethod
    def staticencrypt(secretkey: bytes, buffer: bytes, *, nonce: bytes | None = None, append: bool = True) -> bytes:
        ...
    #
    @staticmethod
    def staticdecrypt(secretkey: bytes, buffer: bytes, *, nonce: bytes | None = None, append: bool = True) -> bytes:
        ...

class XChaCha20Poly1305:
    def __init__(self, secretkey: bytes) -> None:
        ...
    #
    def encrypt(self, buffer: bytes, *, nonce: bytes | None = None, append: bool = True) -> bytes:
        ...
    #
    def decrypt(self, buffer: bytes, *, nonce: bytes | None = None) -> bytes:
        ...
    #
    @staticmethod
    def staticencrypt(secretkey: bytes, buffer: bytes, *, nonce: bytes | None = None, append: bool = True) -> bytes:
        ...
    #
    @staticmethod
    def staticdecrypt(secretkey: bytes, buffer: bytes, *, nonce: bytes | None = None, append: bool = True) -> bytes:
        ...
