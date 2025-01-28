import { ChangeEvent, useCallback } from "react";

interface Props {
  value: number;
  onChange: (value: number) => void;
  min?: number;
  max?: number;
  width?: string;
}

export default ({ value, onChange, min, max, width }: Props) => {
  const handleChange = useCallback(
    (event: ChangeEvent<HTMLInputElement>) => {
      if (event.target.value === "") {
        onChange(min ?? 0);
        return;
      }

      const value = parseInt(event.target.value, 10);
      if (min !== undefined && value < min) {
        onChange(min);
        return;
      }
      if (max !== undefined && value > max) {
        onChange(max);
        return;
      }
      onChange(value);
    },
    [min, max, onChange],
  );

  return (
    <input
      type="number"
      value={value.toFixed(0)}
      onChange={handleChange}
      css={{
        all: "unset",
        backgroundColor: "var(--gray-5)",
        paddingLeft: "var(--space-2)",
        fontSize: "var(--font-size-2)",
        borderRadius: "var(--radius-2)",
        lineHeight: 1.6,
        width,
      }}
    />
  );
};
