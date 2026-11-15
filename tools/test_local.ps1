param(
    [ValidateRange(1, 128)]
    [int]$CpuThreads = [Environment]::ProcessorCount,
    [ValidateRange(1, 16)]
    [int]$GpuThreads = 4,
    [switch]$Release
)

$ErrorActionPreference = 'Stop'
$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$ProfileArgs = @()
if ($Release) { $ProfileArgs = @('--release') }
$Timings = [System.Collections.Generic.List[object]]::new()
$TotalTimer = [System.Diagnostics.Stopwatch]::StartNew()

function Invoke-TestPhase {
    param([string]$Name, [string[]]$TestArgs)
    Write-Host "Testing: $Name"
    $timer = [System.Diagnostics.Stopwatch]::StartNew()
    & cargo test @ProfileArgs @TestArgs
    if ($LASTEXITCODE -ne 0) {
        throw "Test phase '$Name' failed (cargo exit $LASTEXITCODE)."
    }
    $Timings.Add([pscustomobject]@{
        Phase = $Name
        Seconds = [Math]::Round($timer.Elapsed.TotalSeconds, 2)
    })
}

Push-Location $RepoRoot
try {
    # Bake/load the shared font cache once before concurrent app tests. This is
    # one real test; skip it in the later app phase so every test runs once.
    $WarmupTest = 'assets::tests::ambient_labels_prevent_cross_experience_bleed'
    Invoke-TestPhase 'Asset cache warmup' @(
        '-p', 'cosmos_app', '--lib', $WarmupTest, '--', '--exact', '--test-threads=1'
    )

    # Each Cargo test binary runs its independent cases concurrently. Cargo
    # still runs binaries sequentially; GPU jobs cannot overlap timing checks.
    Invoke-TestPhase "CPU, catalog, audio and XR ($CpuThreads threads)" @(
        '--workspace', '--exclude', 'cosmos_app', '--exclude', 'cosmos_render',
        '--exclude', 'cosmos_typo', '--', "--test-threads=$CpuThreads"
    )
    Invoke-TestPhase "Application correctness ($CpuThreads threads)" @(
        '-p', 'cosmos_app', '--lib', '--', "--test-threads=$CpuThreads", '--skip', $WarmupTest
    )
    Invoke-TestPhase 'Guided-tour framing (CPU)' @(
        '-p', 'cosmos_app', '--test', 'tour_framing_tests', '--', "--test-threads=$CpuThreads"
    )
    Invoke-TestPhase "DX12 correctness ($GpuThreads threads)" @(
        '-p', 'cosmos_render', '--lib', '--tests', '--', "--test-threads=$GpuThreads"
    )

    # The typography suite checks cache-load latency; integration_smoke times
    # CPU+GPU frame cadence. Keep both free of competing test workloads.
    Invoke-TestPhase 'Typography and cache timings (isolated)' @(
        '-p', 'cosmos_typo', '--', '--test-threads=1'
    )
    Invoke-TestPhase 'Application integration and benchmark (isolated)' @(
        '-p', 'cosmos_app', '--test', 'integration_smoke', '--', '--test-threads=1'
    )
    Invoke-TestPhase 'Application and renderer doc tests' @(
        '-p', 'cosmos_app', '-p', 'cosmos_render', '--doc'
    )
    $Timings | Format-Table -AutoSize
    Write-Host ('All local tests passed in {0:N2} seconds.' -f $TotalTimer.Elapsed.TotalSeconds)
} finally {
    Pop-Location
}
