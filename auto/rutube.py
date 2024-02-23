import requests
import convert_to_requests

RUTUBE_PLAYLIST_ID = "368049"


print("Step 1: go on Rutube and copy any request as curl")
req = convert_to_requests.curl_to_requests(input("Paste it here or empty to use request.txt: ") or open('request.txt').read())
print(req)

session = requests.Session()
session.headers = req.headers

print("Step 2: check account")

account_data = session.get("https://studio.rutube.ru/api/accounts/visitor/?client=vulp").json()
print('Your email is: ', account_data['email'])

input("continue...")

print("Step 3: preparing video")

upload_data = session.post("https://studio.rutube.ru/api/uploader/upload_session/?client=vulp", json={"cancelToken": {"promise": {}}})
upload_data.raise_for_status()
upload_data = upload_data.json()
sid = upload_data['sid']
videoid = upload_data['video']
print(f"{sid=} {videoid=}")

print("Step 4: setting params")

name = input("Video name:")

print("Step 4: uploading video")

video_path = input("Video path: ")

print("Patching video...")
patch = session.patch(f"https://studio.rutube.ru/api/video/{videoid}/?308&client=vl", data={"title": name, "is_hidden": True, "category": "13"})
print(patch, patch.text)

print("Uploading video... (may take a while)")
req = session.post(f"https://u.rutube.ru/upload/{sid}", files={'data': open(video_path, 'rb')})
print(req, req.text)

print("Step 5: adding to playlist")
p = session.post(f"https://studio.rutube.ru/api/playlist/custom/{RUTUBE_PLAYLIST_ID}", json={"video_ids": [videoid]})

print("Done!")
print("Your video is now at: ")
print(f"\t{patch.json()['video_url']}")

# sid needed for upload to u.rutube.ru/upload/{sid}
# video id needed to watch progress

# Patch studio /api/video/{videoid}&client=vl {"title": "asdf", "is_hidden": true, "category": "13"}
# Get studio /api/uploader/{videoid}/progress/ to see status.