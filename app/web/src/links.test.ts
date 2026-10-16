import { describe, expect, it } from "vitest"
import { PARTY_SITES, PROGRAMMES } from "./links"

describe("candidate links", () => {
  it("are https addresses", () => {
    for (const url of [...Object.values(PARTY_SITES), ...Object.values(PROGRAMMES)]) {
      expect(new URL(url).protocol).toBe("https:")
    }
  })
})
