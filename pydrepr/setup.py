import os
import shutil
import sys

from setuptools import setup, find_packages

wdir = os.path.dirname(os.path.abspath(__file__))
targets = {
    "linux": "x86_64-unknown-linux-gnu",
    "darwin": "x86_64-apple-darwin",
    "win32": "x86_64-pc-windows-msvc"
}

if sys.platform == "win32":
    shutil.copyfile(f"{wdir}/drepr/drepr_engine.{targets[sys.platform]}.pyd",
                    f"{wdir}/drepr/drepr_engine.pyd")
else:
    shutil.copyfile(
        f"{wdir}/drepr/drepr_engine.{targets[sys.platform]}.so", f"{wdir}/drepr/drepr_engine.so")

setup(name="drepr",
      version="2.0a11",
      packages=find_packages(),
      author="Binh Vu",
      author_email="binhlvu@usc.edu",
      license="MIT",
      install_requires=['ujson', 'ruamel.yaml>=0.15.0',
                        'dataclasses;python_version<"3.7"', 'xmltodict'],
      package_data={'': ['*.so', '*.pyd']})
