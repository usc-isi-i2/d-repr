import os
import shutil
import sys
import urllib.request
from setuptools import setup, find_packages
from setuptools.command.install import install

wdir = os.path.dirname(os.path.abspath(__file__))

# load versions
exec(open("drepr/version.py").read())

if sys.argv[1] != "sdist":
    # download correct engine version, only run if this is not the command that build the dist
    target_triple = {
        "linux": "x86_64-unknown-linux-gnu",
        "darwin": "x86_64-apple-darwin",
        "win32": "x86_64-pc-windows-msvc"
    }[sys.platform]
    pylib_ext = {"linux": "so", "darwin": "so", "win32": "pyd"}[sys.platform]
    engine_url = f"https://github.com/usc-isi-i2/d-repr/releases/download/{__engine_release_tag__}/drepr_engine.{target_triple}.{__engine_version__}.{pylib_ext}"
    is_downloaded_test_file = f"{wdir}/download-executed-gn13v9a5.txt"

    if os.path.exists(is_downloaded_test_file):
        print(">>> pre-built engine is already exist! Skip it")
    else:
        print(">>> download pre-built engine at", engine_url)
        urllib.request.urlretrieve(engine_url, f"{wdir}/drepr/drepr_engine.{pylib_ext}")
        with open(is_downloaded_test_file, "w") as f:
            f.write("downloaded")

with open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "README.md"), "r", encoding="utf-8") as f:
    long_description = f.read()

setup(name="drepr",
      version=__version__,
      packages=find_packages(),
      description="Data Representation Language for Reading Heterogeneous Datasets",
      long_description=long_description,
      long_description_content_type='text/markdown',
      author="Binh Vu",
      author_email="binh@toan2.com",
      url="https://github.com/usc-isi-i2/d-repr",
      python_requires='>3.6',
      license="MIT",
      install_requires=['ujson', 'ruamel.yaml>=0.15.0',
                        'dataclasses;python_version<"3.7"', 'xmltodict', 'netcdf4', 'fiona',
                        'pillow'],
      package_data={'': ['*.so', '*.pyd']})
