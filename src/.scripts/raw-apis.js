import{invoke as e}from"@tauri-apps/api/core";var t={fs:Object.freeze({__proto__:null,exists:function(t,i){return e("plugin:apis-fs|exists",{widgetId:t,path:i})},isFile:function(t,i){return e("plugin:apis-fs|is_file",{widgetId:t,path:i})},readFile:function(t,i){return e("plugin:apis-fs|read_file",{widgetId:t,path:i})},writeFile:function(t,i,n){return e("plugin:apis-fs|write_file",{widgetId:t,path:i,content:n})}}),sys:Object.freeze({__proto__:null,getSystemInfo:function(t,i){return e("plugin:apis-sys|get_system_info",{widgetId:t,path:i})}})};export{t as default};
