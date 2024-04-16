# before build, copy `views/canvas-build.html` to `views/canvas.html`

# if canvas exists, remove it
if [ -f views/canvas.html ]; then
  rm views/canvas.html
fi

# copy canvas-build.html to canvas.html
cp views/canvas-build.html views/canvas.html

echo "before build, copied views/canvas-build.html to views/canvas.html"