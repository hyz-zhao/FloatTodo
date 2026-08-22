$WScriptShell = New-Object -ComObject WScript.Shell
$Shortcut = $WScriptShell.CreateShortcut([Environment]::GetFolderPath('Desktop') + '\FloatTodo.lnk')
$Shortcut.TargetPath = Join-Path $PSScriptRoot 'src-tauri\target\release\float-todo.exe'
$Shortcut.WorkingDirectory = $PSScriptRoot
$Shortcut.Save()
Write-Host 'Desktop shortcut created: FloatTodo.lnk'