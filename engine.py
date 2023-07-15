import chess
import ctypes
from helpers import send_command, expect_num_args, expect_at_pos
from search import search


class EngineContext:
    def __init__(self):
        self.settings = {
            "Depth": "7"
        }
        self.debug = False
        self.position = chess.Board()
        self.runLoop = True
        self.execSearch = False


def handle_uci(_context: EngineContext, *args) -> None:
    expect_num_args("uci", [0], *args)
    send_command("id", "name", "BetaFish")
    send_command("id", "author", "Gerald Lee")
    send_command("option", "name", "Depth", "type", "spin", "default", "7", "min", "1", "max", "32")
    send_command("option", "name", "Hash", "type", "spin", "default", "1", "min", "1", "max", "1024")
    send_command("option", "name", "Move Overhead", "type", "spin", "default", "0", "min", "0")
    send_command("option", "name", "Threads", "type", "spin", "default", "1", "min", "1", "max", "128")
    send_command("uciok")


def handle_debug(context: EngineContext, *args) -> None:
    if not expect_num_args("debug", [1], *args):
        return
    if expect_at_pos("debug", 0, ["on", "off"], *args):
        context.debug = args[0] == "on"


def handle_isready(_context: EngineContext, *args) -> None:
    expect_num_args("isready", [0], *args)
    send_command("readyok")


def handle_setoption(context: EngineContext, *args) -> None:
    if not expect_num_args("setoption", [2, 4], *args):
        return
    if not expect_at_pos("setoption", 0, ["name"], *args):
        return
    if len(args) == 4:
        if not expect_at_pos("setoption", 2, ["value"], *args):
            return
        context.settings[args[1]] = args[3]
    else:
        context.settings[args[1]] = None


def handle_ucinewgame(context: EngineContext, *args) -> None:
    expect_num_args("ucinewgame", [0], *args)
    context.position.clear()


def handle_position(context: EngineContext, *args) -> None:
    if not expect_at_pos("position", 0, ["fen", "startpos"], *args):
        return
    if args[0] == "fen":
        context.position.set_fen(args[1])
    elif args[0] == "startpos":
        context.position.reset()
    if len(args) > 1:
        if not expect_at_pos("position", 1, ["moves"], *args):
            return
        for argI in range(2, len(args)):
            context.position.push(chess.Move.from_uci(args[argI]))


def handle_go(context: EngineContext, *_args) -> None:
    send_command("bestmove", search(context.position.fen(), int(context.settings["Depth"])))


def handle_quit(context: EngineContext, *args) -> None:
    expect_num_args("quit", [0], *args)
    context.runLoop = False


handlers = {
    "uci": handle_uci,
    "debug": handle_debug,
    "isready": handle_isready,
    "setoption": handle_setoption,
    "ucinewgame": handle_ucinewgame,
    "position": handle_position,
    "go": handle_go,
    "quit": handle_quit,
}

if __name__ == "__main__":
    context = EngineContext()
    while context.runLoop:
        commandRaw = input()
        if len(commandRaw) == 0:
            continue

        tokens = commandRaw.split(" ")
        command = tokens[0]
        handler = handlers.get(command, None)
        if handler is not None:
            handler(context, *tokens[1:])
        else:
            send_command("info", "string", f"unknown command {command}")
