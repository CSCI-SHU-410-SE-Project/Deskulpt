/**
 * Stringify an unknown error.
 *
 * A string error will be returned as is. An `Error` object will return its
 * stack if available, otherwise its message. If the error does not fall into
 * any of the above categories, a generic message will be returned.
 *
 * @param err The unknown error.
 * @returns The stringified error.
 */
export function stringifyError(err: unknown) {
  if (typeof err === "string") {
    return err;
  }
  if (err instanceof Error) {
    return err.stack ?? err.message;
  }
  return "Unknown error caught that is neither a string nor an Error";
}
