if which cairo-compile; then
  echo "cairo environment installed"
else 
  source ~/cairo_venv/bin/activate
fi
pushd work
date
cairo-compile ../static/samic.cairo --output samic.json
date
cairo-run --program=samic.json --layout=small --print_output
date
cairo-sharp submit --program samic.json 
date
popd
