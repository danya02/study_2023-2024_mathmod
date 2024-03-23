import subprocess
import os

c = 0
for file in os.listdir('.'):
    if file.endswith('-snapshot.json'):
        c = int(file.split('-')[0])
while 1:
    subprocess.check_call(["cargo","run","--release","--",str(c)])
    for file in os.listdir('.'):
        if file.endswith('json'):

            if (file.startswith(str(c-1)+'-')) or(file.startswith(str(c+1)+'-')) or (file.startswith(str(c)+'-')):
                pass
            else:
                subprocess.check_call(['zstd', '-f', '--rm', file])
    c += 1