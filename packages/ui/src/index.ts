export { css, keyframes } from "@emotion/react";

/**
 * Layout
 *
 * The following are not included:
 *
 * - `Container`: used for constraining maximum width of page content; not useful in the
 *   scenario of widgets which have fixed dimensions
 * - `Section`: denotes a section of page content; not useful for the same reason
 */
export { Box, Flex, Grid } from "@radix-ui/themes";

/**
 * Typography
 *
 * All typography components are included.
 */
export {
  Blockquote,
  Code,
  Em,
  Heading,
  Kbd,
  Link,
  Quote,
  Strong,
  Text,
} from "@radix-ui/themes";

/**
 * Components
 *
 * The following are not included:
 *
 * - `AlertDialog`: prefer `Popover`
 * - `Callout`: prefer combination of other components
 * - `Card`: prefer combination of other components
 * - `CheckboxCards`: prefer `Checkbox` and `CheckboxGroup` with custom combination
 * - `Dialog`: prefer `Popover`
 * - `HoverCard`: prefer `Popover`
 * - `Inset`: prefer combination of layout components
 * - `RadioCards`: prefer `Radio` and `RadioGroup` with custom combination
 * - `Skeleton`: unnecessary complexity for a widget
 * - `TabNav`: mostly for navigating between pages, not useful for widgets
 */
export {
  AspectRatio,
  Avatar,
  Badge,
  Button,
  Checkbox,
  CheckboxGroup,
  ContextMenu,
  DataList,
  DropdownMenu,
  IconButton,
  Popover,
  Progress,
  Radio,
  RadioGroup,
  ScrollArea,
  SegmentedControl,
  Select,
  Separator,
  Slider,
  Spinner,
  Switch,
  Table,
  Tabs,
  TextArea,
  TextField,
  Tooltip,
} from "@radix-ui/themes";
