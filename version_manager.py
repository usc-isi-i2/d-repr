import os
import re
from pathlib import Path

# ================================================================================
# Note: update the version by changing variables before the `=...=` line
DREPR_PYLIB_VESRION = "2.9.10"
DREPR_ENGINE_VERSION = "1.0.8"
# this tag marks the release which contains the pre-built engine in it.
DREPR_ENGINE_RELEASE_TAG = "2.7"

# ================================================================================


def update_version(fpath, version, version_tag="version", verbose=True):
    """
    Open the file at `fpath` and find any line with the following pattern:
        `<version_tag> ?= ?"<any_string>" +# ___PKG_VERSION___`
    then, replace `<any_string>` with the `version` argument. The function
    raises exception when no such line is found or more than one line is replaced.

    Example: calling `update_version` with the file has the following content:
    ```
    setup(name="haha",
          version="1.0.0"   # ___PKG_VERSION___
          ...
          )
    ```
    and the replaced `version` is "2.0.0", the file will be updated to:
    ```
    setup(name="haha",
          version="2.0.0"   # ___PKG_VERSION___
          ...
          )
    ```
    """
    with open(fpath, "r") as f:
        content = f.read()

    pattern = '''%s ?= ?["'][^"']*["'] +# ___PKG_VERSION___''' % version_tag
    repl = '%s = "%s"   # ___PKG_VERSION___' % (version_tag, version)
    new_content, n_updates = re.subn(pattern, repl, content)
    assert n_updates == 1, "Cannot update version because there are %s changes to the file" % n_updates

    with open(fpath, "w") as f:
        f.write(new_content)

    if verbose:
        print("Update version of the file: %s" % fpath)
        print("\t+ Original content: %s" % re.findall(pattern, content)[0])
        print("\t+ New content     : %s" % re.findall(pattern, new_content)[0])


if __name__ == "__main__":
    root_dir = Path(os.path.abspath(__file__)).parent
    update_version(root_dir / "drepr" / "engine" / "Cargo.toml", DREPR_ENGINE_VERSION)
    update_version(root_dir / "pydrepr" / "drepr" / "version.py", DREPR_PYLIB_VESRION,
                   "__version__")
    update_version(root_dir / "pydrepr" / "drepr" / "version.py", DREPR_ENGINE_VERSION,
                   "__engine_version__")
    update_version(root_dir / "pydrepr" / "drepr" / "version.py", DREPR_ENGINE_RELEASE_TAG,
                   "__engine_release_tag__")
