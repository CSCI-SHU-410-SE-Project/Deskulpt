import{invoke as i}from"@tauri-apps/api/core";var e={fs:Object.freeze({__proto__:null,exists:function(e,t){return i("plugin:apis-fs|exists",{widgetId:e,path:t})},isFile:function(e,t){return i("plugin:apis-fs|is_file",{widgetId:e,path:t})},readFile:function(e,t){return i("plugin:apis-fs|read_file",{widgetId:e,path:t})},writeFile:function(e,t,r){return i("plugin:apis-fs|write_file",{widgetId:e,path:t,content:r})}})};export{e as default};
