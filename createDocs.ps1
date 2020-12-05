#/usr/bin/env pwsh

Push-Location $PSScriptRoot
try {
    cargo doc --no-deps

    Get-ChildItem target/doc/rusteuler/problem_* | ForEach-Object {
        try{
        $index = "$_/index.html"

        $index -match 'problem_([0-9]+)' > $null
        $problemNumber = $matches[1]

        $runtimeString = (Get-Content -Raw "target/problem_$problemNumber").Trim()

        $newContents = (Get-Content -Raw $index) -replace "====TIME====", "Runs in around $runtimeString seconds"
        Set-Content $index $newContents
        }catch{}
    }
} finally {
    Pop-Location
}
