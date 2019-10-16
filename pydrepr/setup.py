import os
import shutil
import sys
import urllib.request
from setuptools import setup, find_packages

# load versions
exec(open("drepr/version.py").read())

# download correct engine version

# wdir = os.path.dirname(os.path.abspath(__file__))
# targets = {
#     "linux": "x86_64-unknown-linux-gnu",
#     "darwin": "x86_64-apple-darwin",
#     "win32": "x86_64-pc-windows-msvc"
# }

# if sys.platform == "win32":
#     shutil.copyfile(f"{wdir}/drepr/drepr_engine.{targets[sys.platform]}.pyd",
#                     f"{wdir}/drepr/drepr_engine.pyd")
# else:
#     shutil.copyfile(
#         f"{wdir}/drepr/drepr_engine.{targets[sys.platform]}.so", f"{wdir}/drepr/drepr_engine.so")

setup(name="drepr",
      version=__version__,
      packages=find_packages(),
      author="Binh Vu",
      author_email="binh@toan2.com",
      python_requires='>3.6',
      license="MIT",
      install_requires=['ujson', 'ruamel.yaml>=0.15.0',
                        'dataclasses;python_version<"3.7"', 'xmltodict'],
      package_data={})
