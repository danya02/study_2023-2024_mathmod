import os
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

if os.system(f'./youtubeuploader -filename "{work}" -metaJSON meta.json -metaJSONout work.json'): exit()

work_meta = json.load(open("work.json"))

meta = {
    "title": f'Лаб {lab_number} презентация',
    "playlistIds": ["PLr7fi2vP83YFZEpwoitd9XlZlJHFREI3s"]
}

open('meta.json','w').write(json.dumps(meta))

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
print("Go to: https://studio.rutube.ru/uploader")
print("Upload the following files with the following names:")
print("\t", work, "\t", f"Лаб {lab_number} выполнение")
print("\t", present, "\t", f"Лаб {lab_number} презентация")
print("Make sure to set them to unlisted as you upload them.")

rutube_work_url = input("Please paste the first URL here: ")
rutube_present_url = input("Please paste the second URL here: ")

os.system("clear")

print("Step 5: Rutube playlist")
print("Go to:")
print(f"\thttps://studio.rutube.ru/playlist/{RUTUBE_PLAYLIST_ID}")
print("and add the two new videos there.")
print("(They will not appear until they are processed and moderated.)")
print("(If you already specified the playlist when uploading, you may skip waiting for this step.)")


input("Press Enter to continue to building documents...")

print("Step 6: build documents")

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

print("Step 7: collecting files")

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

print("Step 8: Github Release")
print("Go to:")
print(f"\thttps://github.com/{GH_REPO}/releases/new")
print("then create a new release.")
print("Upload all files from:")
print("\t", gh)

release_tag = input("Please enter the release tag here (like v0.1.0): ")

os.system("clear")

print("Step 9: Moodle submission")
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