param(
    [string]$Tag = "v0.1.0",
    [string]$Repo = "RONITERVO/HyperCosmosVR",
    [string]$ExpectedSha256 = "EAB2B3B9D19EFB2D165002B9D1A027ADC4931B3D40B78400D76A8BF1791B77CF",
    [string]$Destination = "",
    [switch]$Force
)

$ErrorActionPreference = "Stop"

$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
if ([string]::IsNullOrWhiteSpace($Destination)) {
    $Destination = Join-Path $RepoRoot "assets\baked\universe_audio.hcvp"
}

$DestDir = Split-Path -Parent $Destination
if (-not (Test-Path $DestDir)) {
    New-Item -ItemType Directory -Force -Path $DestDir | Out-Null
}

function Test-HashMatch([string]$Path, [string]$Expected) {
    if ([string]::IsNullOrWhiteSpace($Expected)) {
        return $true
    }
    $actual = (Get-FileHash -Algorithm SHA256 -Path $Path).Hash.ToUpperInvariant()
    return ($actual -eq $Expected.ToUpperInvariant())
}

if ((Test-Path $Destination) -and (-not $Force)) {
    Write-Host "[fetch_audio_pack] Found existing audio pack at $Destination; verifying SHA-256..."
    if (Test-HashMatch -Path $Destination -Expected $ExpectedSha256) {
        Write-Host "[fetch_audio_pack] Verified SHA-256 matches ($ExpectedSha256). Nothing to download."
        exit 0
    }
    Write-Warning "[fetch_audio_pack] Existing file hash mismatch; re-downloading..."
}

$TmpPath = "$Destination.part"
if (Test-Path $TmpPath) {
    Remove-Item -Force $TmpPath
}

$GhCmd = Get-Command gh -ErrorAction SilentlyContinue
if ($GhCmd) {
    Write-Host "[fetch_audio_pack] Downloading universe_audio.hcvp (~1.05 GiB) from GitHub Release $Tag via gh..."
    & gh release download $Tag --repo $Repo --pattern "universe_audio.hcvp" --output $TmpPath --clobber
    if ($LASTEXITCODE -ne 0) {
        throw "gh release download failed with exit code $LASTEXITCODE"
    }
} else {
    $Url = "https://github.com/$Repo/releases/download/$Tag/universe_audio.hcvp"
    Write-Host "[fetch_audio_pack] Downloading universe_audio.hcvp (~1.05 GiB) from $Url..."
    $ProgressPreference = "SilentlyContinue"
    Invoke-WebRequest -Uri $Url -OutFile $TmpPath -UseBasicParsing
}

if (-not (Test-HashMatch -Path $TmpPath -Expected $ExpectedSha256)) {
    $actual = (Get-FileHash -Algorithm SHA256 -Path $TmpPath).Hash
    Remove-Item -Force $TmpPath -ErrorAction SilentlyContinue
    throw "SHA-256 mismatch for downloaded universe_audio.hcvp! Expected $ExpectedSha256, got $actual"
}

Move-Item -Path $TmpPath -Destination $Destination -Force
$SizeMiB = [math]::Round((Get-Item $Destination).Length / 1MB, 2)
Write-Host "[fetch_audio_pack] Verified & installed universe_audio.hcvp ($SizeMiB MiB) at $Destination."

