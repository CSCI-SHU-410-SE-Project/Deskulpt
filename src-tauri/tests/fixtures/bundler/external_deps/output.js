import React from"@deskulpt-test/react";import osName from"os-name";import{matcher}from"matcher";const __default={render(){return React.createElement("div",null,React.createElement("p",null,"Your OS: ",osName()),React.createElement("p",null,"Matcher: ",matcher(["foo","bar","baz"],"b*")));},width:100,height:100};export{__default as default};