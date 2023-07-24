from subprocess import Popen, PIPE


searchProcess = Popen("target/release/beta_fish", stdin=PIPE, stdout=PIPE, universal_newlines=True)


def search(fen: str, depth: int) -> str:
    print(f"fen {fen}", file=searchProcess.stdin, flush=True)
    print(f"depth {depth}", file=searchProcess.stdin, flush=True)
    print(f"eval", file=searchProcess.stdin, flush=True)
    return searchProcess.stdout.readline().strip("\n")
