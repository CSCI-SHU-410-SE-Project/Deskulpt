import { LuFolderOpen } from "react-icons/lu";
import FloatButton from "./FloatButton";
import { useCallback } from "react";
import { invokeOpenInWidgetsDir } from "../../../core/commands";

export default () => {
  const action = useCallback(() => {
    invokeOpenInWidgetsDir({ components: [] }).catch(console.error);
  }, []);

  return (
    <FloatButton
      bottom="0"
      icon={<LuFolderOpen />}
      tooltip="Open widgets directory"
      onClick={action}
    />
  );
};
