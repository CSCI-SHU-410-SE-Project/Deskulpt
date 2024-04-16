# before dev, copy `views/canvas-dev.html` to `views/canvas.html`

# if canvas exists, remove it
if [ -f views/canvas.html ]; then
  rm views/canvas.html
fi

# copy canvas-dev.html to canvas.html
cp views/canvas-dev.html views/canvas.html

echo "before dev, copied views/canvas-dev.html to views/canvas.html"