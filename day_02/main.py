import logging
import math
import os

import debugpy
import logging
import colorlog


_logger = logging.getLogger(__name__)

def setup_pretty_logging():
    logging.basicConfig(level=logging.INFO, format="[%(asctime)s][%(filename)s:%(lineno)d][%(levelname)s]: %(message)s")
    # Try using colorlog
    logging.getLogger().handlers[0].setFormatter(
        colorlog.ColoredFormatter(
            "[%(cyan)s%(asctime)s%(reset)s][%(blue)s%(filename)s:%(lineno)d%(reset)s][%(log_color)s%(levelname)s%(reset)s]: %(message)s"
        )
    )


def get_invalid_ids(start, end):
    invalid_ids = []
    for i in range(start, end+1):
        if check_invalid(i):
            invalid_ids.append(i)
    return invalid_ids

def clamp(x, a, b):
    if x <= a:
        return a
    elif x >= b:
        return b
    else:
        return x

def split_string(str_num, count):
    if count == len(str_num):
        assert False
        return []
    if len(str_num) % count != 0:
        return []

    all_splits = []
    for i in range(0, len(str_num), count):
        all_splits.append(str_num[i:i+count])
    return all_splits

def check_invalid(num) -> bool:
    str_num = str(num)
    if len(str_num) == 1:
        return False

    max_val = clamp( math.ceil(len(str_num) / 2) + 1, 2, len(str_num))
    for i in range(1, max_val):
        splits = split_string(str_num, i)
        if len(set(splits)) == 1:
            return True

    return False

def main():
    _logger.info("Start")
    # file_name = "input_eg.txt"
    # file_name = "input_simple.txt"
    file_name = "input_full.txt"
    with open(file_name) as f:
        lines = f.readlines()
    
    values = []
    for line in lines:
        slices = line.rstrip("\n").split(",")
        for s in slices:
            start, _, end = s.partition("-")
            values.append((int(start), int(end)))

    invalid_ids = []
    for start, end in values:
        invalid_ids.extend(get_invalid_ids(start, end))
    
    print(f"invalid_ids:")
    for i in invalid_ids:
        print(f"  {i}")

    print("")
    print(f"sum: {sum(invalid_ids)}")

if __name__ == "__main__":
    if os.environ.get("ENABLE_DEBUGPY") is not None:
        print("Waiting for debugger to connect")
        debugpy.listen(("127.0.0.1", 5678))
        debugpy.wait_for_client()
    setup_pretty_logging()
    main()
