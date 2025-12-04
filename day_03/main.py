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

def get_max(digits: list[int], start, end):
    assert end-start > 0
    index = start
    for i in range(start+1, end):
        if digits[i] > digits[index]:
            index = i
    return index, digits[index]

def get_jolts(power_bank: list[int], num_batteries: int):
    assert len(power_bank) >= num_batteries

    indices = []
    digits = []
    for i in range(num_batteries):
        battery_start = indices[i-1] + 1 if i >= 1 else 0
        battery_end = len(power_bank) - (num_batteries - i - 1)

        index, digit = get_max(power_bank, battery_start, battery_end)
        indices.append(index)
        digits.append(digit)
    

    str_jolt = "".join((str(_) for _ in digits))
    jolt = int(str_jolt)
    return jolt
    # first_digit = 0
    # for i in range(1, len(power_bank) - 1):
    #     if power_bank[i] > power_bank[first_digit]:
    #         first_digit = i
    
    # second_digit = first_digit+1
    # for j in range(first_digit+2, len(power_bank)):
    #     if power_bank[j] > power_bank[second_digit]:
    #         second_digit = j
    
    # jolt = 10 * power_bank[first_digit] + power_bank[second_digit]
    return jolt


def main():
    _logger.info("Start")
    # file_name = "input_eg.txt"
    # file_name = "input_simple.txt"
    file_name = "input_full.txt"
    with open(file_name) as f:
        lines = f.readlines()
    
    power_banks = []
    for line in lines:
        line = line.rstrip("\n")
        nums = []
        for digit in line:
            nums.append(int(digit))
        power_banks.append(nums)


    all_jolts = 0
    for power_bank in power_banks:
        jolt = get_jolts(power_bank, 12)
        all_jolts += jolt

        power_bank_str = "".join([str(_) for _ in power_bank])
        print(f"{power_bank_str} --> {jolt}")

    print("")
    print(f"Total jolts: {all_jolts}")


if __name__ == "__main__":
    if os.environ.get("ENABLE_DEBUGPY") is not None:
        print("Waiting for debugger to connect")
        debugpy.listen(("127.0.0.1", 5678))
        debugpy.wait_for_client()
    setup_pretty_logging()
    main()
