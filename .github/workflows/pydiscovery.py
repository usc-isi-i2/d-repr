from __future__ import print_function
import os, subprocess, re


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
        output = (
            subprocess.check_output([os.path.join(home, "bin", "python"), "-V"])
            .decode()
            .strip()
        )
        for version in versions:
            m = re.match("Python ([\d\.)]+)", output)
            assert m is not None
            pyversion = m.group(1)
            if pyversion.startswith(version):
                filtered_homes.append(home)
                break

    homes = filtered_homes


if "MINIMUM_PYTHON_VERSION" in os.environ:
    minimum_version = [int(d) for d in os.environ["MINIMUM_PYTHON_VERSION"].split(".")]
    filtered_homes = []
    for home in homes:
        output = (
            subprocess.check_output([os.path.join(home, "bin", "python"), "-V"])
            .decode()
            .strip()
        )
        m = re.match(r"Python ([\d\.)]+)", output)
        assert m is not None
        pyversion = m.group(1).split(".")

        if all(
            int(pyversion[i]) >= minimum_version[i] for i in range(len(minimum_version))
        ):
            filtered_homes.append(home)

    homes = filtered_homes

print(",".join(homes))
exit(0)
