from helpers import send_command


def handle_uci(*args) -> None:
    if len(args) > 0:
        send_command(
            "info",
            "string",
            f"Command uci has incorrect number of args. Found {len(args)}, expected 0"
        )
    send_command("id", "name", "BetaFish")
    send_command("id", "author", "Gerald Lee")
    send_command("uciok")


handlers = {
    "uci": handle_uci,
}

if __name__ == "__main__":
    while True:
        commandRaw = input()
        if len(commandRaw) == 0:
            continue

        tokens = commandRaw.split(" ")
        command = tokens[0]
        handler = handlers.get(command, None)
        if handler is not None:
            handler(*tokens[1:])
        else:
            send_command("info", "string", f"unknown command {command}")
