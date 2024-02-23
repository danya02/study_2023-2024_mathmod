import os

import convert_to_requests
import requests
lab_number = int(input("What is the lab number? "))
print("Step 1: make videos")

GH_REPO = "danya02/study_2023-2024_mathmod"

YOUTUBE_PLAYLIST_ID = "PLr7fi2vP83YFZEpwoitd9XlZlJHFREI3s"
# https://www.youtube.com/playlist?list=PLr7fi2vP83YFZEpwoitd9XlZlJHFREI3s

RUTUBE_PLAYLIST_ID = "368049"
# https://rutube.ru/plst/368049

work = os.path.abspath(input("Path to work video: "))
present = os.path.abspath(input("Path to present video: "))

print("Step 2: upload videos to Youtube")
print("Make sure that https://github.com/porjo/youtubeuploader/releases is in the current directory")
print("Also, make sure that it is already authorized as an OAuth app and has your account access (request.token exists).")
print("Try uploading any video file using the program and follow the steps.")
print()
input("Press Enter to continue with uploading...")

import json
meta = {
    "title": f'Лаб {lab_number} выполнение',
    "playlistIds": [YOUTUBE_PLAYLIST_ID]
}

open('meta.json','w').write(json.dumps(meta))

if os.path.exists("work.json"):
    print("Skipping upload to Youtube of work")
else:
    if os.system(f'./youtubeuploader -filename "{work}" -metaJSON meta.json -metaJSONout work.json'): exit()

work_meta = json.load(open("work.json"))

meta = {
    "title": f'Лаб {lab_number} презентация',
    "playlistIds": [YOUTUBE_PLAYLIST_ID]
}

open('meta.json','w').write(json.dumps(meta))


if os.path.exists("present.json"):
    print("Skipping upload to Youtube of present")
else:
    if os.system(f'./youtubeuploader -filename "{present}"  -metaJSON meta.json -metaJSONout present.json'): exit()

present_meta = json.load(open("present.json"))

os.system("clear")

print("Step 3: making videos viewable")
print("Go to:")
print(f"\thttps://studio.youtube.com/playlist/{YOUTUBE_PLAYLIST_ID}/videos")
print("and make the new videos unlisted instead of private.")

input("Press Enter to continue to Rutube...")

os.system("clear")

print("Step 4: Upload to Rutube")
print("4.1: go on Rutube and copy any request as curl")
req = convert_to_requests.curl_to_requests(input("Paste it here or empty to use request.txt: ") or open('request.txt').read())

session = requests.Session()
session.headers = req.headers

print("4.2: check account")

account_data = session.get("https://studio.rutube.ru/api/accounts/visitor/?client=vulp").json()
print('Your email is: ', account_data['email'])
input("Press Enter to continue...")


print("4.3: preparing videos")
upload_data = session.post("https://studio.rutube.ru/api/uploader/upload_session/?client=vulp", json={"cancelToken": {"promise": {}}})
upload_data.raise_for_status()
upload_data = upload_data.json()
sid_work = upload_data['sid']
videoid_work = upload_data['video']
print(f"{sid_work=} {videoid_work=}")

upload_data = session.post("https://studio.rutube.ru/api/uploader/upload_session/?client=vulp", json={"cancelToken": {"promise": {}}})
upload_data.raise_for_status()
upload_data = upload_data.json()
sid_present = upload_data['sid']
videoid_present = upload_data['video']
print(f"{sid_present=} {videoid_present=}")

print("4.4: setting params")
patch_work = session.patch(f"https://studio.rutube.ru/api/video/{videoid_work}/?308&client=vl", data={"title": f"Лабораторная работа {lab_number} выполнение", "is_hidden": True, "category": "13", "is_adult": True})
patch_work.raise_for_status()
rutube_work_info = patch_work.json()

patch_present = session.patch(f"https://studio.rutube.ru/api/video/{videoid_present}/?308&client=vl", data={"title": f"Лабораторная работа {lab_number} выполнение", "is_hidden": True, "category": "13", "is_adult": True})
patch_present.raise_for_status()
rutube_present_info = patch_work.json()

rutube_work_url = rutube_work_info['video_url']
rutube_present_url = rutube_present_info['video_url']

print("4.5 upload data (may take a while)")
print("Uploading work...")
req = session.post(f"https://u.rutube.ru/upload/{sid_work}", files={'data': open(work, 'rb')})
print(req, req.text)

print("Uploading present...")
req = session.post(f"https://u.rutube.ru/upload/{sid_present}", files={'data': open(present, 'rb')})
print(req, req.text)

print("4.6 add to playlist")
p = session.post(f"https://studio.rutube.ru/api/playlist/custom/{RUTUBE_PLAYLIST_ID}", json={"video_ids": [videoid_work]})
print(p, p.text)
p = session.post(f"https://studio.rutube.ru/api/playlist/custom/{RUTUBE_PLAYLIST_ID}", json={"video_ids": [videoid_present]})
print(p, p.text)


input("Press enter to continue...")

os.system("clear")

input("Press Enter to continue to building documents...")

print("Step 5: build documents")

cwd = os.getcwd()
os.chdir(f"../labs/lab{lab_number}/report")
os.system("make")
os.chdir("../presentation")
makefile = open("Makefile").read()
makefile = makefile.replace("xelatex", "lualatex")
open("Makefile", "w").write(makefile)
os.system("make")

os.chdir("..")

os.system("clear")

print("Step 6: collecting files")

gh = os.path.join(cwd, "github")
mo = os.path.join(cwd, "moodle")
os.makedirs(gh)
os.makedirs(mo)

import shutil
shutil.copy("report/report.md", gh)  # отчёт в markdown (в каталоге git и в файлах релиза);
shutil.copy("report/report.docx", gh)  # отчёт в docx (сделанный из markdown) (приложено к ответу и в файлах релиза);
shutil.copy("report/report.docx", mo)
shutil.copy("report/report.pdf", gh)  # отчёт в pdf (сделанный из markdown) (приложено к ответу и в файлах релиза);
shutil.copy("report/report.pdf", mo)

os.system(f"zip -r /tmp/lab{lab_number}.zip *")
shutil.copy(f"/tmp/lab{lab_number}.zip", mo)  # архив с исходными материалами markdown (текстовые файлы, скриншоты и т. д.);
shutil.copy(f"/tmp/lab{lab_number}.zip", gh)

shutil.copy("presentation/presentation.pdf", gh)  # презентацию в pdf и html (сделанные из markdown) (приложено к ответу и в файлах релиза);
shutil.copy("presentation/presentation.html", gh)
shutil.copy("presentation/presentation.pdf", mo)
shutil.copy("presentation/presentation.html", mo)

shutil.copy("presentation/presentation.md", gh)  # презентацию в markdown (в каталоге git и в файлах релиза).

os.system("clear")

print("Step 7: Github Release")
print("Go to:")
print(f"\thttps://github.com/{GH_REPO}/releases/new")
print("then create a new release.")
print("Upload all files from:")
print("\t", gh)

release_tag = input("Please enter the release tag here (like v0.1.0): ")

os.system("clear")

print("Step 8: Moodle submission")
print("Go to:")
print("\t https://esystem.rudn.ru/course/view.php?id=10676")
print("and draft a new submission for lab", lab_number)

print("Upload all files from:")
print("\t", mo)

print("Use this as body text:")
print("---")

print("Лаб", lab_number)
print()
print("YOUTUBE:")
print(f"Плейлист: https://www.youtube.com/playlist?list={YOUTUBE_PLAYLIST_ID}")
print(f"Выполнение: https://youtu.be/{work_meta['id']}")
print(f"Подготовка отчета: https://youtu.be/{work_meta['id']}")
print(f"Подготовка презентации: https://youtu.be/{work_meta['id']}")
print(f"Презентация: https://youtu.be/{present_meta['id']}")
print("RUTUBE:")
print(f"Плейлист: https://rutube.ru/plst/{RUTUBE_PLAYLIST_ID}")
print(f"Выполнение: {rutube_work_url}")
print(f"Подготовка отчета: {rutube_work_url}")
print(f"Подготовка презентации: {rutube_work_url}")
print(f"Презентация: {rutube_present_url}")

print("GITHUB:")
print(f"Репозиторий: https://github.com/{GH_REPO}")
print(f"Релиз: https://github.com/{GH_REPO}/releases/tag/{release_tag}")

print()
print("---")
print("When you're done, you can clean up with:")
print()
print("rm -r", gh)
print("rm -r", mo)
print("rm ./meta.json ./work.json ./present.json")
print("rm", work)
print("rm", present)