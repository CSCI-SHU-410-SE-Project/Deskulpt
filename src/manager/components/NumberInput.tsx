import { ChangeEvent } from "react";

interface NumberInputProps {
  /** The controlled number input value. */
  value: number;
  /** The callback on value change. */
  onChange: (value: number) => void;
  /** The minimal accepted number. */
  min?: number;
  /** The maximal accepted number. */
  max?: number;
  /** The widget of the number input area. */
  width?: string;
}

/**
 * A simple number input component.
 *
 * Compared with the feature-rich number input components from libraries like Material
 * UI and Ant Design, this component is not for general use but designed only for
 * specific use cases in this project.
 *
 * The component is *controlled*, meaning that the value is passed in as a prop and the
 * parent component is responsible for updating it. The component is also functional,
 * such that it has the increment/decrement buttons, accepts keyboard input, reacts to
 * up/down keys and the scroll wheel, etc.
 */
const NumberInput = ({
  value,
  onChange,
  min,
  max,
  width,
}: NumberInputProps) => {
  function handleChange(event: ChangeEvent<HTMLInputElement>) {
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
};

export default NumberInput;
