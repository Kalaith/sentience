<#
.SYNOPSIS
    Headless screenshot harness for Sentience.

.DESCRIPTION
    Thin wrapper around the shared macroquad-toolkit capture script. Builds the
    debug exe and drives it through the env-var capture hook (SENTIENCE_CAPTURE_*)
    provided by macroquad_toolkit::capture in src/main.rs. The boot flow lands
    directly in gameplay at level 0, so "gameplay" needs no extra seeding;
    "decision" and "ending" force the session into those overlay states.

.EXAMPLE
    ./scripts/capture_ui.ps1
    ./scripts/capture_ui.ps1 -Scenes gameplay,decision,ending -Frames 150
#>
param(
    [string[]]$Scenes = @("gameplay", "decision", "ending"),
    [int]$Frames = 150,
    [string]$OutputDir = "docs\verification",
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$gameDir = Split-Path -Parent $PSScriptRoot
$shared = Join-Path (Split-Path -Parent $gameDir) "macroquad-toolkit\scripts\capture_ui.ps1"

& $shared -GameDir $gameDir -Prefix "SENTIENCE" -Scenes $Scenes -Frames $Frames -OutputDir $OutputDir -SkipBuild:$SkipBuild
