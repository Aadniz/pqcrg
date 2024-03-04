{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell rec {
  name = "impurePythonEnv";
  venvDir = "venv";
  buildInputs = [
  ] ++ (with python3Packages; [
    python
    semgrep
    ruff-lsp
    python-lsp-server
    poetry-core
  ]);

  # This is very close to how venvShellHook is implemented, but
  # adapted to use 'virtualenv'
  shellHook = ''
    git submodule init
    git submodule update
    cd library/pqcrypto
    poetry build
    python3 compile.py
    python3 extend.py
    cd ../../


    SOURCE_DATE_EPOCH=$(date +%s)

    if [ -d "${venvDir}" ]; then
      echo "Skipping venv creation, '${venvDir}' already exists"
    else
      echo "Creating new venv environment in path: '${venvDir}'"
      # Note that the module venv was only introduced in python 3, so for 2.7
      # this needs to be replaced with a call to virtualenv
      ${python3Packages.python.interpreter} -m venv "${venvDir}"
    fi

    # Under some circumstances it might be necessary to add your virtual
    # environment to PYTHONPATH, which you can do here too;
    PYTHONPATH=$PWD/${venvDir}/${python3Packages.python.sitePackages}/:$PYTHONPATH

    source "${venvDir}/bin/activate"

    ${venvDir}/bin/pip install -r requirements.txt
  '';
}
