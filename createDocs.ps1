#/usr/bin/env pwsh

function GetProblemNumber {
    [CmdletBinding()]
    [OutputType([String])]
    param(
        [Parameter(Mandatory)]
        [String] $S
    )

    $S -match 'problem_([0-9]+)' > $null
    return $matches[1]
}

Push-Location $PSScriptRoot
try {
    cargo doc --no-deps

    Get-ChildItem target/doc/rusteuler/problem_* | ForEach-Object {
        $index = "$_/index.html"

        $problemNumber = GetProblemNumber $index
        $runtimeString = (Get-Content -Raw "target/problem_$problemNumber").Trim()

        $newContents = (Get-Content -Raw $index) -replace "====TIME====", "Runs in around $runtimeString seconds"
        Set-Content $index $newContents
    }

    Get-ChildItem static/problem_* | ForEach-Object {
        $file = $_

        $problemNumber = GetProblemNumber $file
        Copy-Item $file "target/doc/rusteuler/problem_$problemNumber"
    }
} finally {
    Pop-Location
}
