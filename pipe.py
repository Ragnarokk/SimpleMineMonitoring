import sys, tty

# fd = sys.stdin.fileno()
# tty.setraw(fd)

# a = input()
# print(a)
print("Hello worlds")

while(True):
    ch = sys.stdin.read(1)

    if ch == 'q':
        break

    sys.stdout.write(ch*2)
    sys.stdout.flush()

# tty.setcbreak(fd)