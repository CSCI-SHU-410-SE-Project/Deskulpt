import { ChangeEvent, useEffect } from "react";

interface NumberInputProps {
  value: number;
  onChange: (value: number) => void;
  min?: number;
  max?: number;
  width?: string;
}

export default function NumberInput({
  value,
  onChange,
  min,
  max,
  width,
}: NumberInputProps) {
  useEffect(() => {
    onChange && onChange(value);
  }, [value]);

  function handleChange(event: ChangeEvent<HTMLInputElement>) {
    if (event.target.value === "") {
      onChange(min ?? 0);
      return;
    }

    const value = parseInt(event.target.value);
    if (min !== undefined && value < min) {
      onChange(min);
      return;
    }
    if (max !== undefined && value > max) {
      onChange(max);
      return;
    }
    onChange(value);
  }

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
        lineHeight: 1.6,
        borderRadius: "var(--radius-2)",
        width,
      }}
    />
  );
}
