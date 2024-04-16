# if `dist/@deskulpt/react/index.js` exists, append `export default React`
# if `dist/@deskulpt/apis/index.js` exists, append `export default apis`

# React
if [ -f dist/@deskulpt/react/index.js ]; then
  echo "export default React" >> dist/@deskulpt/react/index.js
fi

# apis
if [ -f dist/@deskulpt/apis/index.js ]; then
  echo "export default apis" >> dist/@deskulpt/apis/index.js
fi

echo "Post vite build script executed"