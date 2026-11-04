#!/usr/bin/env python3
"""Build release archives from an already tested native executable (stdlib only)."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import stat
import tarfile
import tempfile
import zipfile

ROOT = Path(__file__).resolve().parents[1]
TARGETS = {
    'x86_64-unknown-linux-gnu': 'linux-x86_64',
    'x86_64-pc-windows-msvc': 'windows-x86_64',
    'aarch64-apple-darwin': 'macos-arm64',
    'x86_64-apple-darwin': 'macos-x86_64',
}

def version():
    return re.search(r'^version = "([^"]+)"', (ROOT/'Cargo.toml').read_text(), re.M).group(1)

def zip_tree(folder, output):
    with zipfile.ZipFile(output, 'w', zipfile.ZIP_DEFLATED) as archive:
        for path in sorted(folder.rglob('*')):
            if not path.is_file():
                continue
            relative = path.relative_to(folder.parent).as_posix()
            info = zipfile.ZipInfo(relative, date_time=(1980, 1, 1, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            info.create_system = 3
            info.external_attr = (stat.S_IFREG | stat.S_IWUSR | stat.S_IRUSR) << 16
            with path.open('rb') as source:
                archive.writestr(info, source.read())

def checksums(output):
    paths = sorted(p for p in output.iterdir() if p.suffix == '.zip' or p.name.endswith('.tar.gz'))
    (output/'SHA256SUMS.txt').write_text(''.join(f'{hashlib.sha256(p.read_bytes()).hexdigest()}  {p.name}\n' for p in paths), encoding='utf-8')

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--target', choices=TARGETS)
    parser.add_argument('--binary', type=Path)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--starter-only', action='store_true')
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory() as temporary:
        base = Path(temporary)
        starter = base/'dqp-starter'
        shutil.copytree(ROOT/'starter', starter)
        starter_out = args.out/f'dqp-starter-v{version()}.zip'
        zip_tree(starter, starter_out)
        if not args.starter_only:
            if args.target is None or args.binary is None:
                parser.error('--target and --binary are required unless --starter-only is used')
            is_windows = 'windows' in args.target
            name = f'dqp-v{version()}-{TARGETS[args.target]}'
            package = base/name
            package.mkdir()
            binary = package/('dqp.exe' if is_windows else 'dqp')
            shutil.copy2(args.binary, binary)
            binary.chmod(0o755)
            shutil.copytree(ROOT/'starter', package/'starter')
            shutil.copy2(ROOT/'LICENSE', package/'LICENSE')
            shutil.copy2(ROOT/'docs/INSTALLATION.md', package/'INSTALLATION.md')
            installer = 'install.ps1' if is_windows else 'install.sh'
            shutil.copy2(ROOT/'scripts'/installer, package/installer)
            # Include license texts from the actual locked dependency sources.
            metadata = json.loads(subprocess.check_output(
                ['cargo','metadata','--locked','--format-version','1','--filter-platform',args.target], cwd=ROOT))
            notices = ['DQP third-party dependencies', 'Rust standard library: MIT OR Apache-2.0; https://github.com/rust-lang/rust', '']
            for dependency in sorted(metadata['packages'], key=lambda p:p['id']):
                if dependency['name'] == 'dqp': continue
                label = f"{dependency['name']}-{dependency['version']}"
                notices.append(f"{label}: {dependency.get('license') or 'See accompanying license'}; {dependency.get('repository') or ''}")
                manifest = Path(dependency['manifest_path']).parent
                license_paths = list(manifest.glob('LICENSE*')) + list(manifest.glob('COPYING*'))
                if dependency.get('license_file'):
                    license_paths.append(manifest/dependency['license_file'])
                for source in sorted(set(license_paths)):
                    if source.is_file():
                        dest = package/'licenses'/label/source.name
                        dest.parent.mkdir(parents=True,exist_ok=True)
                        shutil.copy2(source,dest)
            # Standard-library copyright notices, vendored from the toolchain source.
            for source in (ROOT/'packaging/licenses').glob('*'):
                dest=package/'licenses/rust'/source.name
                dest.parent.mkdir(parents=True,exist_ok=True)
                shutil.copy2(source,dest)
            (package/'THIRD_PARTY_NOTICES.txt').write_text('\n'.join(notices)+'\n',encoding='utf-8')
            if is_windows: zip_tree(package,args.out/(name+'.zip'))
            else:
                with tarfile.open(args.out/(name+'.tar.gz'),'w:gz') as archive:
                    archive.add(package,arcname=name)
        checksums(args.out)
    for path in sorted(args.out.iterdir()): print(path.name)

if __name__ == '__main__': main()
