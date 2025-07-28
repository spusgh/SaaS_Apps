/#

-- to kill all ollama process - repeat for each process id
PS C:\Users\Shail> Get-Process -Name ollama | Select-Object -Property Id

   Id
   --
16272


PS C:\Users\Shail> Stop-Process -Id 16272


#/

taskkill /F /IM ollama.exe
Stop-Service -Name "ollama"
Start-Process "ollama" -ArgumentList "serve" -WindowStyle Hidden
 
ollama start
ollama list

