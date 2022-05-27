from pathlib import Path
import subprocess, shutil

cwd = Path(".")

print("Build webapp")
subprocess.check_call("npm run build", cwd=str(cwd), shell=True)

print("Copy webapp files...")
shutil.rmtree(cwd / "../api/flask_files/static/webapp")
shutil.copytree(cwd / "build/static", cwd / "../api/flask_files/static/webapp")

print("Re-compile index.html")
with open(cwd / "../api/flask_files/templates/index.html.template", "r") as f:
    html = f.read()

    for file in (cwd / "../api/flask_files/static/webapp/css").iterdir():
        if not file.name.endswith(".css"):
            continue

        if file.name.startswith("1."):
            html = html.replace("%1.chunk.css%", file.name)
        if file.name.startswith("main."):
            html = html.replace("%main.chunk.css%", file.name)

    for file in (cwd / "../api/flask_files/static/webapp/js").iterdir():
        if not file.name.endswith(".js"):
            continue

        if file.name.startswith("1."):
            html = html.replace("%1.chunk.js%", file.name)
        if file.name.startswith("main."):
            html = html.replace("%main.chunk.js%", file.name)
        if file.name.startswith("runtime"):
            html = html.replace("%runtime~main.js%", file.name)

with open(cwd / "../api/flask_files/templates/index.html", "w") as f:
    f.write(html)
