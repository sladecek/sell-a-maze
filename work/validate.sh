if which cairo-compile; then
  echo "cairo environment installed"
else 
  source ~/cairo_venv/bin/activate
fi
pushd work
cairo-compile ../static/samic.cairo --output samic.json
cairo-run --program=samic.json --layout=small --print_output
cairo-sharp submit --program samic.json 
popd
