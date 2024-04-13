import { useState, useEffect } from "react";
// import JsxParser from 'react-jsx-parser';

import Button from "@mui/material/Button";

// Material UI component
// import Textfield from '@mui/material/TextField';

// import { invoke } from '@tauri-apps/api/tauri';
// import { LogicalSize, appWindow } from '@tauri-apps/api/window';

// import MultiWindowManager from './MultiWindowManager/MultiWindowManager';
import WidgetManager from "./WidgetManager/WidgetManager";

// to pease tsc and ignore unmatched type
// const JsxParserComponent = JsxParser as any;

// const myScriptTSX = `
//   <div>
//     <p>This is rendered by <code>TSX</code> userscript.</p>
//     <h2>Counter</h2>
//     <Counter />
//   </div>
// `

// Predefined element with props
// const EchoText = (props: { value: string }) => {
//   return <p>{props.value}</p>;
// }

// Predefined element with state
const Counter = () => {
  const [count, setCount] = useState(0);
  return (
    <div>
      <p>Count: {count}</p>
      <Button variant="contained" color="primary" onClick={() => setCount(count + 1)}>
        Increment
      </Button>
    </div>
  );
};

// await appWindow.setSize(new LogicalSize(
//   document.body.scrollWidth, document.body.scrollHeight
// ));

// async function setWindowMinSize( { width, height }: { width: number, height: number }) {
//   const curSize = await appWindow.innerSize();
//   // console.log(curSize);
//   // console.log(width, height);
//   const newSize = new LogicalSize(
//     Math.min(width, curSize.width),
//     Math.min(height, curSize.height)
//   );
//   // console.log(await appWindow.innerSize());
//   await appWindow.setSize(newSize);
// }

// const UserComponent = ( {userscript}: { userscript: string } ) => {
//   return (
//     <JsxParserComponent
//         bindings={{
//           myEventHandler: () => { console.log("click") }
//         }}
//         components={{ Counter, Textfield, EchoText, Button }}
//         jsx={userscript}
//       />
//   );
// }

const App = () => {
  useEffect(() => {
    // Call the default tauri api to adjust the window size
    console.log("UseEffect called in App.tsx");
  }, []);

  return (
    <div>
      {/* To drag the window in tauri, see this Github discussion: https://github.com/tauri-apps/tauri/discussions/4362 */}
      <div data-tauri-drag-region>Drag area</div>
      {/* <UserComponent userscript={myScriptTSX} /> */}
      <h2>Counter</h2>
      <Counter />
      <WidgetManager />
    </div>
  );
};
export default App;
