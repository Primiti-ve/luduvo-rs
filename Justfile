set shell := ["powershell.exe", "-Command"]

push title description:
    git add -A
    git commit -m "{{ title }}" -m "{{ description }}"
    git push origin main

release version description:
    git add -A
    git commit -m "{{ version }}" -m "{{ description }}"
    git tag "{{ version }}"
    git push origin main --tags --force

test:
    cargo test --tests
