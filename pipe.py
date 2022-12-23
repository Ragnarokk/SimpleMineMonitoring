import sys, tty
from time import sleep

# fd = sys.stdin.fileno()
# tty.setraw(fd)

# a = input()
# print(a)

for _ in range(500):
    print("Hello worlds")
    sleep(1)

while(True):
    ch = sys.stdin.read(1)

    if ch == 'q':
        break

    sys.stdout.write(ch*2)
    sys.stdout.flush()

# tty.setcbreak(fd)