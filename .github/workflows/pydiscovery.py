from __future__ import print_function
import os, subprocess


homes = []
if "PYTHON_HOME" in os.environ:
    homes.append(os.environ["PYTHON_HOME"])
elif "PYTHON_HOMES" in os.environ:
    lst = os.environ["PYTHON_HOMES"].split(",")
    for path in lst:
        if os.path.exists(os.path.join(path, "bin", "python")):
            # is the python directory
            homes.append(path)
        else:
            for subpath in os.listdir(path):
                if os.path.exists(os.path.join(path, subpath, "bin", "python")):
                    homes.append(os.path.join(path, subpath))

if "PYTHON_VERSIONS" in os.environ:
    versions = os.environ["PYTHON_VERSIONS"].split(",")
    filtered_homes = []
    for home in homes:
        output = subprocess.check_output([os.path.join(home, "bin", "python"), "-V"]).decode().strip()
        for version in versions:
            if output.replace("Python ", "").startswith(version):
                filtered_homes.append(home)
                break

    homes = filtered_homes

print(",".join(homes))
exit(0)