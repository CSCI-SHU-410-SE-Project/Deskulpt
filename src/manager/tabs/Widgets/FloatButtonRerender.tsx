import { LuRepeat } from "react-icons/lu";
import FloatButton from "./FloatButton";
import { WidgetsState } from "../../hooks";
import { useCallback } from "react";
import { emitRenderToCanvas } from "../../../core/events";
import { toast } from "sonner";

interface Props {
  widgets: WidgetsState;
}

export default ({ widgets }: Props) => {
  const action = useCallback(() => {
    emitRenderToCanvas(widgets.map(({ id, settings }) => ({ id, settings })))
      .then(() => {
        toast.success(`Re-rendered ${widgets.length} widgets.`);
      })
      .catch(console.error);
  }, [widgets]);

  return (
    <FloatButton
      bottom="80px"
      icon={<LuRepeat />}
      tooltip="Re-render current widgets"
      onClick={action}
      disabled={widgets.length === 0}
    />
  );
};
