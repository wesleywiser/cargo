import distutils.spawn
import hashlib
import os
import subprocess
import sys
import tarfile
import shutil
import contextlib

with open('src/snapshots.txt') as f:
    lines = f.readlines()

date = lines[0]
linux32 = lines[1]
linux64 = lines[2]
mac32 = lines[3]
mac64 = lines[4]
win32 = lines[5]
win64 = lines[6]
triple = sys.argv[1]

ts = triple.split('-')
arch = ts[0]
if len(ts) == 2:
    vendor = 'unknown'
    target_os = ts[1]
else:
    vendor = ts[1]
    target_os = ts[2]

intel32 = (arch == 'i686') or (arch == 'i586')

me = None
if target_os == 'linux':
    if intel32:
        me = linux32
        new_triple = 'i686-unknown-linux-gnu'
    elif arch == 'x86_64':
        me = linux64
        new_triple = 'x86_64-unknown-linux-gnu'
elif target_os == 'darwin':
    if intel32:
        me = mac32
        new_triple = 'i686-apple-darwin'
    elif arch == 'x86_64':
        me = mac64
        new_triple = 'x86_64-apple-darwin'
elif target_os == 'windows':
    if intel32:
        me = win32
        new_triple = 'i686-pc-windows-gnu'
    elif arch == 'x86_64':
        me = win64
        new_triple = 'x86_64-pc-windows-gnu'

if me is None:
    raise Exception("no snapshot for the triple: " + triple)

triple = new_triple

platform, hash = me.strip().split()

tarball = 'cargo-nightly-' + triple + '.tar.gz'
url = 'https://static-rust-lang-org.s3.amazonaws.com/cargo-dist/' + date.strip() + '/' + tarball
dl_path = "target/dl/" + tarball
dst = "target/snapshot"

if not os.path.isdir('target/dl'):
    os.makedirs('target/dl')

if os.path.isdir(dst):
    shutil.rmtree(dst)

exists = False
if os.path.exists(dl_path):
    h = hashlib.sha1(open(dl_path, 'rb').read()).hexdigest()
    if h == hash:
        print("file already present %s (%s)" % (dl_path, hash,))
        exists = True

if not exists:
    ret = subprocess.call(["curl", "-o", dl_path, url])
    if ret != 0:
        raise Exception("failed to fetch url")
    h = hashlib.sha1(open(dl_path, 'rb').read()).hexdigest()
    if h != hash:
        raise Exception("failed to verify the checksum of the snapshot")

with contextlib.closing(tarfile.open(dl_path)) as tar:
    for p in tar.getnames():
        name = p.replace("cargo-nightly-" + triple + "/", "", 1)
        fp = os.path.join(dst, name)
        print("extracting " + p)
        tar.extract(p, dst)
        tp = os.path.join(dst, p)
        if os.path.isdir(tp) and os.path.exists(fp):
            continue
        shutil.move(tp, fp)
shutil.rmtree(os.path.join(dst, 'cargo-nightly-' + triple))
