set shell := ["powershell.exe", "-Command"]

push title description:
    git add -A
    git commit -m "{{title}}" -m "{{description}}"
    git push origin main

release version:
    git add -A
    git commit -m "{{version}}"
    git tag "{{version}}"
    git push origin main --tags
