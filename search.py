import ctypes


searchLib = ctypes.CDLL("target/release/libbeta_fish_search.so")
searchLib.start_search.restype = ctypes.c_char_p


def search(fen: str, depth: int) -> str:
    result = searchLib.start_search(fen.encode("utf-8"), depth.to_bytes(2, byteorder="little", signed=False))
    return ctypes.cast(result, ctypes.c_char_p).value.decode("utf-8")
