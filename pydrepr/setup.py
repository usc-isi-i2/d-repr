import os
import shutil
import sys
import urllib.request
from setuptools import setup, find_packages
from setuptools.command.install import install

wdir = os.path.dirname(os.path.abspath(__file__))

# load versions
exec(open("drepr/version.py").read())

class custom_install(install):
    def run(self):
        # download correct engine version
        target_triple = {
            "linux": "x86_64-unknown-linux-gnu",
            "darwin": "x86_64-apple-darwin",
            "win32": "x86_64-pc-windows-msvc"
        }[sys.platform]
        pylib_ext = {"linux": "so", "darwin": "so", "win32": "pyd"}[sys.platform]
        engine_file = f"https://github.com/usc-isi-i2/d-repr/releases/download/{__engine_release_tag__}/drepr_engine.{target_triple}.{__engine_version__}.{pylib_ext}"

        print(">>> download pre-built engine at", engine_file)
        urllib.request.urlretrieve(engine_file, f"{wdir}/drepr/drepr_engine.{pylib_ext}")

        # then run the normal installation process
        install.run(self)

with open(os.path.join(wdir, "README.md"), "r", encoding="utf-8") as f:
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
      cmdclass={
          "install": custom_install
      },
      install_requires=['ujson', 'ruamel.yaml>=0.15.0',
                        'dataclasses;python_version<"3.7"', 'xmltodict'],
      package_data={'': ['*.so', '*.pyd']})
