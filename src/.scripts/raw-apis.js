import{invoke as i}from"@tauri-apps/api";var t=Object.freeze({__proto__:null,exists:function(t,e){return i("plugin:widget_api.fs|exists",{widgetId:t,path:e})},isFile:function(t,e){return i("plugin:widget_api.fs|is_file",{widgetId:t,path:e})},readFile:function(t,e){return i("plugin:widget_api.fs|read_file",{widgetId:t,path:e})},writeFile:function(t,e,n){return i("plugin:widget_api.fs|write_file",{widgetId:t,path:e,content:n})}});export{t as fs};
