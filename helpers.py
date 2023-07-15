from typing import Iterable


def send_command(command: str, *args) -> None:
    print(command, *args)


def expect_num_args(command: str, nums: Iterable[int], *args) -> bool:
    if len(args) not in nums:
        send_command(
            "info",
            "string",
            (
                f"Command {command} has incorrect number of args. Found {len(args)},"
                f" expected {', or '.join(str(num) for num in nums)}"
            ),
        )
        return False
    return True


def expect_at_pos(command: str, pos: int, expected: Iterable[str], *args) -> bool:
    if len(args) <= pos:
        send_command(
            "info",
            "string",
            (
                f"Command {command} expected one of '{', '.join(expected)}' as argument"
                f" {pos}, but only {len(args)} args were provided"
            ),
        )
        return False
    elif args[pos] not in expected:
        send_command(
            "info",
            "string",
            (
                f"Command {command} expected one of '{', '.join(expected)}' as argument"
                f" {pos}, but '{args[pos]}' was found instead"
            ),
        )
        return False
    return True
