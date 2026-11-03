#!/usr/bin/env python3
import sys
from package import version
expected = 'v'+version()
if len(sys.argv) != 2 or sys.argv[1] != expected:
    raise SystemExit('Tag attendu : '+expected)
print('Version cohérente : '+expected)
