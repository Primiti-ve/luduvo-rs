set shell := ["powershell.exe", "-Command"]

push title description:
    git add -A
    git commit -m "{{title}}" -m "{{description}}"
    git tag "{{title}}"
    git push origin main --tags
