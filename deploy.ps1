cargo doc --no-deps
echo "<meta http-equiv=\`"refresh\`" content=\`"0; url=build_wheel\`">" > target/doc/index.html
# cp -r target/doc ./docs
Copy-Item -Path ".\target\doc" -Destination ".\docs" -recurse -Force
git stage -A
git commit -a -m "deployment"