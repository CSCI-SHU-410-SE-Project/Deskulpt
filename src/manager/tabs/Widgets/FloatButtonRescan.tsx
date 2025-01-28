import { LuFileScan } from "react-icons/lu";
import FloatButton from "./FloatButton";
import { RescanCallback } from "../../hooks";
import { useCallback } from "react";
import { toast } from "sonner";

interface Props {
  rescan: RescanCallback;
}

export default ({ rescan }: Props) => {
  const action = useCallback(() => {
    rescan()
      .then(({ numAdded, numRemoved, numUpdated }) => {
        toast.success(
          `${numAdded} added, ${numRemoved} removed, ${numUpdated} refreshed.`,
        );
      })
      .catch(console.error);
  }, [rescan]);

  return (
    <FloatButton
      bottom="40px"
      icon={<LuFileScan />}
      tooltip="Rescan widgets directory"
      onClick={action}
    />
  );
};
