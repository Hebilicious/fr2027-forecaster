import { describe, expect, it } from "vitest"
import { cadenceMessages, messages } from "./i18n"

describe("cadenceMessages", () => {
  it("renders every cadence code in French and English", () => {
    expect(Object.keys(cadenceMessages).toSorted()).toEqual(["6h", "daily", "hourly", "on_publication"])
    expect(messages.en[cadenceMessages["6h"]]).toBe("every 6 hours")
    expect(messages.fr[cadenceMessages["6h"]]).toBe("toutes les 6 heures")
    expect(messages.fr[cadenceMessages.on_publication]).toBe("à chaque sondage publié")
  })
})
