import logging
import math
import os
import os.path as op
import numpy as np
import torch
import debugpy
from pathlib import Path
from matplotlib import cm
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

def calc_count_smart(values, old_intermediate):
    cur_pos = 50
    counter = 0
    intermediate = []

    for i, value in enumerate(values):
        new_pos_without_mod = (cur_pos + value)
        remainder = new_pos_without_mod % 100
        factor = (new_pos_without_mod - remainder) // 100
    
        if factor <= 0 and remainder == 0:
            factor -= 1
        if cur_pos == 0 and factor < 0 and remainder != 0:
            factor += 1

        counter += abs(factor)
        
        print (f"{i:5d} cur: {cur_pos:5d}, value: {value:5d}, new: {new_pos_without_mod:5d}, clicks: {abs(factor):5d}, cumulative: {counter:5d}")
        if old_intermediate[i] != counter:
            a = 1

        cur_pos = remainder
    
    return counter, intermediate

def sign(x):
    assert x != 0
    return 1 if x > 0 else -1

def calc_count(values):
    cur_pos = 50
    counter = 0
    intermediate = []
    for i, value in enumerate(values):
        
        for j in range(abs(value)):
            cur_pos = (cur_pos + sign(value)) % 100
            if cur_pos == 0:
               counter += 1 
        # print (f"{i:5d} cur: {cur_pos:5d}, value: {value:5d}")
        intermediate.append(counter)
    return counter, intermediate

def main():
    _logger.info("Start")
    # file_name = "input.txt"
    # file_name = "input_base.txt"
    # file_name = "input_harshit.txt"
    file_name = "input_simple_2.txt"
    with open(file_name) as f:
        lines = f.readlines()
    
    values = []
    for line in lines:
        line = line.rstrip("\n")
        dir = (1 if line[0].lower() == "r" else -1)
        num = int(line[1:])
        
        assert num != 0
        values.append(num * dir)

    counter_1, intermediate_1 = calc_count(values)
    counter_2, intermediate_2 = calc_count_smart(values, intermediate_1)
    
    print(f"counter_1: {counter_1}, counter_2: {counter_2}")
    

if __name__ == "__main__":
    if os.environ.get("ENABLE_DEBUGPY") is not None:
        print("Waiting for debugger to connect")
        debugpy.listen(("127.0.0.1", 5678))
        debugpy.wait_for_client()
    setup_pretty_logging()
    main()
