import{invoke as i}from"@tauri-apps/api/core";var n={fs:Object.freeze({__proto__:null,appendFile:function(n,e){return i("call_plugin",{plugin:"fs",command:"append_file",widgetId:n,payload:e})},createDir:function(n,e){return i("call_plugin",{plugin:"fs",command:"create_dir",widgetId:n,payload:e})},exists:function(n,e){return i("call_plugin",{plugin:"fs",command:"exists",widgetId:n,payload:e})},isDir:function(n,e){return i("call_plugin",{plugin:"fs",command:"is_dir",widgetId:n,payload:e})},isFile:function(n,e){return i("call_plugin",{plugin:"fs",command:"is_file",widgetId:n,payload:e})},readFile:function(n,e){return i("call_plugin",{plugin:"fs",command:"read_file",widgetId:n,payload:e})},removeDir:function(n,e){return i("call_plugin",{plugin:"fs",command:"remove_dir",widgetId:n,payload:e})},removeFile:function(n,e){return i("call_plugin",{plugin:"fs",command:"remove_file",widgetId:n,payload:e})},writeFile:function(n,e){return i("call_plugin",{plugin:"fs",command:"write_file",widgetId:n,payload:e})}}),sys:Object.freeze({__proto__:null,getSystemInfo:function(n){return i("call_plugin",{plugin:"sys",command:"get_system_info",widgetId:n})}})};export{n as default};
