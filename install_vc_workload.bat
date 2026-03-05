
@echo off
echo Installing Desktop development with C++ workload...
"C:\Program Files (x86)\Microsoft Visual Studio\Installer\setup.exe" modify --installPath "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools" --add Microsoft.VisualStudio.Workload.VCTools;includeRecommended --passive --wait
echo Done! Please restart the build.
