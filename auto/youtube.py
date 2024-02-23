import os

YOUTUBE_PLAYLIST_ID = "PLr7fi2vP83YFZEpwoitd9XlZlJHFREI3s"
# https://www.youtube.com/playlist?list=PLr7fi2vP83YFZEpwoitd9XlZlJHFREI3s

vid = os.path.abspath(input("What to upload: "))

name = input("What to name it:")

print("Make sure that https://github.com/porjo/youtubeuploader/releases is in the current directory")
print("Also, make sure that it is already authorized as an OAuth app and has your account access (request.token exists).")
print("Try uploading any video file using the program and follow the steps.")
print()
input("Press Enter to continue with uploading...")

import json
meta = {
    "title": name,
    "playlistIds": [YOUTUBE_PLAYLIST_ID]
}

open('meta.json','w').write(json.dumps(meta))

if os.system(f'./youtubeuploader -filename "{vid}" -metaJSON meta.json -metaJSONout vid.json'): exit()

vid_meta = json.load(open("vid.json"))

print("Now go to: ")
print(f"\thttps://studio.youtube.com/playlist/{YOUTUBE_PLAYLIST_ID}/videos")
print("and make the new video unlisted instead of private.")

print()

print("After that, your new video can be viewed at: ")
print(f"\thttps://youtu.be/{vid_meta['id']}")
print()
print("Make sure to clean up:")
print("\trm meta.json vid.json")