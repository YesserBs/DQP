$ErrorActionPreference = 'Stop'
$source = Join-Path $PSScriptRoot 'dqp.exe'
$destination = Join-Path $env:LOCALAPPDATA 'DQP\bin'
$binary = Join-Path $destination 'dqp.exe'
if (-not (Test-Path $source)) { throw 'Binaire dqp.exe absent : extrayez le paquet Windows complet.' }
if (Test-Path $binary) { throw "Installation existante : $binary. Renommez-la avant de la remplacer." }
& $source --version
if ($LASTEXITCODE -ne 0) { throw 'Ce binaire ne fonctionne pas sur ce système.' }
New-Item -ItemType Directory -Force -Path $destination | Out-Null
Copy-Item -LiteralPath $source -Destination $binary
Write-Host "DQP installé dans $destination"
Write-Host 'Pour ce terminal :'
Write-Host ('$env:Path = "' + $destination + ';" + $env:Path')
Write-Host 'Pour une installation permanente, ajoutez ce dossier au Path utilisateur dans les variables d’environnement Windows, puis rouvrez VS Code.'
