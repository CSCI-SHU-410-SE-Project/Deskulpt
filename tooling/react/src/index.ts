import React from "react";

/**
 * React hooks; https://18.react.dev/reference/react/hooks
 *
 * The following hooks are not included:
 * - useActionState: experimental
 * - useDebugValue: only related to react devtools which will not be available
 * - useOptimistic: experimental
 */
import {
  useCallback,
  useContext,
  useDeferredValue,
  useEffect,
  useId,
  useImperativeHandle,
  useInsertionEffect,
  useLayoutEffect,
  useMemo,
  useReducer,
  useRef,
  useState,
  useSyncExternalStore,
  useTransition,
} from "react";

/**
 * React components; https://18.react.dev/reference/react/components
 *
 * The following components are not included:
 * - Profiler: disabled in production build by default
 * - StrictMode: only used for development, and widgets are already wrapped in strict
 *   mode in the widget canvas
 */
import { Fragment, Suspense } from "react";

/**
 * APIs; https://18.react.dev/reference/react/apis
 *
 * The following APIs are not included:
 * - act: only used for testing
 * - cache: experimental
 * - use: experimental
 */
import { createContext, forwardRef, lazy, memo, startTransition } from "react";

export default React;

export {
  useCallback,
  useContext,
  useDeferredValue,
  useEffect,
  useId,
  useImperativeHandle,
  useInsertionEffect,
  useLayoutEffect,
  useMemo,
  useReducer,
  useRef,
  useState,
  useSyncExternalStore,
  useTransition,
  Fragment,
  Suspense,
  createContext,
  forwardRef,
  lazy,
  memo,
  startTransition,
};
