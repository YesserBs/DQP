#!/usr/bin/env python3
"""Merge native CI outputs; duplicate starter archives must have equal contents."""
import hashlib
from pathlib import Path
import shutil
import sys
import zipfile
from package import checksums

def signature(path):
    if path.suffix=='.zip':
        with zipfile.ZipFile(path) as z:
            return sorted((name,hashlib.sha256(z.read(name)).hexdigest()) for name in z.namelist() if not name.endswith('/'))
    return hashlib.sha256(path.read_bytes()).hexdigest()

source, output = map(Path, sys.argv[1:])
output.mkdir(parents=True,exist_ok=True)
for path in sorted(source.rglob('*')):
    if not path.is_file() or not (path.name.endswith('.zip') or path.name.endswith('.tar.gz')): continue
    dest=output/path.name
    if dest.exists():
        if signature(dest)!=signature(path): raise SystemExit('Paquets divergents : '+path.name)
    else: shutil.copy2(path,dest)
if len(list(output.glob('*.zip'))) + len(list(output.glob('*.tar.gz'))) != 5:
    raise SystemExit('Cinq paquets attendus : quatre plateformes et un starter.')
checksums(output)
