/**
 * Grab as much information as possible from the unknown error.
 *
 * A string error will be returned as is. An Error object will return its stack if it
 * exists, otherwise its message. An unknown error will return a generic message.
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
