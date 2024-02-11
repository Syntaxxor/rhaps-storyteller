mkdir ".packaging"
mkdir ".packaging/rhaps-storyteller"

robocopy "target/release/" ".packaging/rhaps-storyteller" "rhaps-storyteller.exe"
robocopy "stories" ".packaging/rhaps-storyteller/stories" /e
robocopy "resources" ".packaging/rhaps-storyteller/resources" /e

powershell -command "Compress-Archive -Path '.packaging/rhaps-storyteller/' -DestinationPath 'packaged.zip' -Force"

rmdir ".packaging" /S /Q