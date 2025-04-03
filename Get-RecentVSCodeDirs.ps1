$storage_json_path = "$Env:APPDATA\Code\User\globalStorage\storage.json"
$storage_json = Get-Content -Raw $storage_json_path | ConvertFrom-Json -AsHashtable
