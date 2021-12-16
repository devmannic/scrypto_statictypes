#!/usr/bin/env python3
import sys
import os
import re

def main(argv=None):
    if argv is None:
        argv = sys.argv

    if len(argv) < 2:
        print(f'Usage: {argv[0]} <version>')
        return 1

    new_tag = argv[1];
    M = re.match('v?(\d.*)', new_tag)
    if not M:
        print(f"bad version: {argv[1]}")
        print(f'Usage: {argv[0]} <version>')
        return 1

    new_version = M.group(1)

    if len(argv) > 2 and argv[2] == '--start':
        me = os.path.abspath(sys.argv[0])
        r = os.system(f'find . -iname Cargo.toml -execdir {me} {new_tag} \;') ;
        return r

    # bump package version
    r = os.system(f'cargo set-version {new_version}')
    if r != 0:
        return r
    # bump all scrypto dependencies to match
    scrypto_tag = new_tag
    scrypto_url = 'https://github.com/radixdlt/radixdlt-scrypto'
    scrypto_deps = [
        'scrypto',
        'sbor',
        'radix-engine',
    ]
    with open('Cargo.toml') as fobj:
        lines = fobj.readlines()
    lines = list(lines)
    for i, line in enumerate(lines):
        line = line.strip()
        for dep in scrypto_deps:
            match = f'{dep} = {{ git = "{scrypto_url}"'
            if line.startswith(match):
                line = match + f', tag = "{scrypto_tag}" }}'
                break
        lines[i] = line + "\n"
    with open('Cargo.toml', 'w') as fobj:
        fobj.writelines(lines)
    return 0

if __name__ == '__main__':
    sys.exit(main())
