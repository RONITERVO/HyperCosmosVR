param(
    [switch]$VerifyOnly,
    [switch]$RequireAudioPack
)

$ErrorActionPreference = "Stop"

$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path

function Find-DxcDir {
    $preferred = "C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64"
    if ((Test-Path (Join-Path $preferred "dxcompiler.dll")) -and (Test-Path (Join-Path $preferred "dxil.dll"))) {
        return $preferred
    }

    $kitsRoot = "C:\Program Files (x86)\Windows Kits\10\bin"
    if (Test-Path $kitsRoot) {
        $candidates = Get-ChildItem -Path $kitsRoot -Directory | Sort-Object Name -Descending
        foreach ($dir in $candidates) {
            $x64 = Join-Path $dir.FullName "x64"
            if ((Test-Path (Join-Path $x64 "dxcompiler.dll")) -and (Test-Path (Join-Path $x64 "dxil.dll"))) {
                return $x64
            }
        }
    }
    return $null
}

$DxcDir = Find-DxcDir
if (-not $DxcDir) {
    Write-Error "Could not locate Windows Kits 10 x64 dxcompiler.dll and dxil.dll under C:\Program Files (x86)\Windows Kits\10\bin"
    exit 1
}

$RequiredAssets = @(
    "assets\baked\universe_catalog.hcvb",
    "assets\hrir\irc_1040_c.bin",
    "assets\textures\blue_noise_128.png",
    "shaders\common_oled.wgsl",
    "shaders\compute_universe.wgsl",
    "shaders\far_shell.wgsl",
    "shaders\hero_volumetric.wgsl",
    "shaders\instanced_sprites.wgsl",
    "shaders\msdf_text.wgsl",
    "shaders\post_bloom_agx.wgsl"
)

# Stage Windows system fonts locally if not already provisioned (keeps proprietary .ttf files out of Git).
$FontsDir = Join-Path $RepoRoot "assets\fonts"
if (-not (Test-Path $FontsDir)) {
    New-Item -ItemType Directory -Force -Path $FontsDir | Out-Null
}
$FontMap = @{
    "display_script.ttf"      = "segoesc.ttf"
    "display_script_bold.ttf" = "segoescb.ttf"
    "ui_sans.ttf"             = "segoeui.ttf"
    "ui_sans_bold.ttf"        = "segoeuib.ttf"
    "symbols_fallback.ttf"    = "seguisym.ttf"
}
$WinFontsDir = Join-Path $env:WINDIR "Fonts"
$MissingFonts = @()
foreach ($targetName in $FontMap.Keys) {
    $dest = Join-Path $FontsDir $targetName
    if (-not (Test-Path $dest)) {
        $src = Join-Path $WinFontsDir $FontMap[$targetName]
        if (Test-Path $src) {
            Copy-Item -Path $src -Destination $dest -Force
            Write-Host "[stage_runtime_deps] Staged local font $targetName from $src"
        } else {
            $MissingFonts += "$targetName (expected Windows font: $($FontMap[$targetName]))"
        }
    }
}
if ($MissingFonts.Count -gt 0) {
    Write-Warning ("Some local fonts were not found in $WinFontsDir and must be placed manually in assets\fonts\ (see assets\fonts\README.md):`n - " + ($MissingFonts -join "`n - "))
}

$MissingAssets = @()
foreach ($rel in $RequiredAssets) {
    $full = Join-Path $RepoRoot $rel
    if (-not (Test-Path $full)) {
        $MissingAssets += $rel
    }
}
if ($MissingAssets.Count -gt 0) {
    Write-Error ("Missing required runtime assets:`n - " + ($MissingAssets -join "`n - "))
    exit 1
}

$AudioPack = Join-Path $RepoRoot "assets\baked\universe_audio.hcvp"
if (-not (Test-Path $AudioPack)) {
    $msg = "Optional 1.05 GiB narration audio pack (assets\baked\universe_audio.hcvp) is not present. " +
           "The app runs in visual + spatial synth mode without it; run 'powershell -ExecutionPolicy Bypass -File tools\fetch_audio_pack.ps1' " +
           "(or 'python tools\bake_universe.py') to enable full 18,898-asset spoken narration."
    if ($RequireAudioPack) {
        Write-Error $msg
        exit 1
    } else {
        Write-Warning $msg
    }
}

$MsdfBin = Join-Path $RepoRoot "assets\baked\msdf_cache\msdf_atlas_v1.bin"
if (-not (Test-Path $MsdfBin)) {
    Write-Host "[stage_runtime_deps] MSDF glyph atlas cache not pre-baked; first launch will automatically bake assets\baked\msdf_cache\ from assets\fonts\*.ttf."
}

$TargetDirs = @(
    (Join-Path $RepoRoot "target\release"),
    (Join-Path $RepoRoot "target\debug"),
    (Join-Path $RepoRoot "target\release\deps"),
    (Join-Path $RepoRoot "target\debug\deps")
)

$DllNames = @("dxcompiler.dll", "dxil.dll")

if (-not $VerifyOnly) {
    foreach ($dir in $TargetDirs) {
        if (-not (Test-Path $dir)) {
            New-Item -ItemType Directory -Path $dir -Force | Out-Null
        }
        foreach ($dll in $DllNames) {
            $src = Join-Path $DxcDir $dll
            $dst = Join-Path $dir $dll
            $needCopy = $true
            if (Test-Path $dst) {
                $srcInfo = Get-Item $src
                $dstInfo = Get-Item $dst
                if ($srcInfo.Length -eq $dstInfo.Length -and $srcInfo.LastWriteTimeUtc -eq $dstInfo.LastWriteTimeUtc) {
                    $needCopy = $false
                }
            }
            if ($needCopy) {
                Copy-Item -Path $src -Destination $dst -Force
            }
        }
    }
    Write-Host "[stage_runtime_deps] Staged dxcompiler.dll & dxil.dll from '$DxcDir' into target/{debug,release}{,/deps}."
} else {
    Write-Host "[stage_runtime_deps] Verified DXC DLLs at '$DxcDir'."
}

Write-Host "[stage_runtime_deps] Verified all $($RequiredAssets.Count) baked asset & shader dependencies under '$RepoRoot'."
exit 0

