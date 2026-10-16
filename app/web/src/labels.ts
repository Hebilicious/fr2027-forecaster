/**
 * The label a dictionary gives a code sent by the data (a collector name, an event kind), or the code itself when
 * the dictionary has none: a code added upstream shows as sent until it is translated.
 */
export function labelOf(labels: object, code: string): string {
  if (!Object.keys(labels).includes(code)) return code
  return String((labels as Record<string, { value: unknown }>)[code]?.value ?? code)
}
