.\scripts\build_docs.ps1
rm -r -Force ./docs
echo "<meta http-equiv=`"refresh`" content=`"0; url=kat`">" > target/doc/index.html
Copy-Item -Path ".\target\doc" -Destination ".\docs" -recurse -Force