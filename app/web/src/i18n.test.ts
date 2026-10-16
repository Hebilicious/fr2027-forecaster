import { describe, expect, it } from "vitest"
import { cadenceMessages, messages } from "./i18n"

describe("cadenceMessages", () => {
  it("renders every cadence code in French and English", () => {
    expect(Object.keys(cadenceMessages).toSorted()).toEqual(["12h", "daily", "hourly", "on_publication"])
    expect(messages.en[cadenceMessages["12h"]]).toBe("every 12 hours")
    expect(messages.fr[cadenceMessages["12h"]]).toBe("toutes les 12 heures")
    expect(messages.fr[cadenceMessages.on_publication]).toBe("à chaque sondage publié")
  })
})
