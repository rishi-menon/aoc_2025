
import debugpy
import os


def main():
    file_path = "input_simple.txt"
    file_path = "input_full.txt"
    with open(file_path) as f:
        lines = f.readlines()

    cur_laser = [1 if _ == "S" else 0 for _ in lines[0]]
    for splitter in lines[1:]:
        splitter = splitter.strip("\n")
        new_laser = [0 for _ in range(len(cur_laser))]
        for i, splitter_char in enumerate(splitter):
            if splitter_char == "^" and cur_laser[i] > 0:
                if i-1 >= 0:
                    new_laser[i-1] += cur_laser[i]
                if i+1 < len(cur_laser):
                    new_laser[i+1] += cur_laser[i]
            else:
                new_laser[i] += cur_laser[i]
        
        cur_laser = new_laser
    
    print(f"Total timelines: {sum(cur_laser)}")






if __name__ == "__main__":
    if os.environ.get("ENABLE_DEBUGPY") is not None:
        print("Waiting for debugger to connect")
        debugpy.listen(("127.0.0.1", 5678))
        debugpy.wait_for_client()
    main()




