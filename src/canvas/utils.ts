/**
 * Grab as much information as possible from an unknown error.
 *
 * A string error will be returned as is. An `Error` object will return its stack if it
 * exists, otherwise its message. If the error does not fall into any of the above
 * categories, a generic message will be returned.
 *
 * @param err The unknown error, commonly from `catch`.
 * @returns The error information.
 */
export function grabErrorInfo(err: unknown) {
  if (typeof err === "string") {
    return err;
  }
  if (err instanceof Error) {
    return err.stack ?? err.message;
  }
  return "Unknown error caught that is neither a string nor an Error";
}
