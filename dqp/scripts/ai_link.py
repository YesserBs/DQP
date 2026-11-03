#!/usr/bin/env python3
"""Print a version-pinned public documentation URL for a supplied GitHub repository."""
import argparse
import re
from package import version
p=argparse.ArgumentParser(description=__doc__)
p.add_argument('repository',help='https://github.com/owner/repository')
a=p.parse_args()
match=re.fullmatch(r'https://github\.com/([A-Za-z0-9_.-]+)/([A-Za-z0-9_.-]+?)(?:\.git)?/?',a.repository)
if not match: p.error('URL GitHub HTTPS de dépôt attendue.')
owner,repo=match.groups()
url=f'https://github.com/{owner}/{repo}/blob/v{version()}/docs/v1/AI_GUIDE.md'
print('Après publication du tag et si le dépôt est public :')
print(url)
print('\nPrompt :')
print(f'Lis cette documentation : {url}. Génère uniquement un scénario DQP 1.0 pour réviser les listes Python, niveau débutant, avec un rattrapage sur les indices. Si tu ne peux pas accéder au guide, indique-le avant de générer le fichier.')
