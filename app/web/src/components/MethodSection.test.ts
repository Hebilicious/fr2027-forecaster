import { readFileSync } from "node:fs"
import { describe, expect, it } from "vitest"
import method from "./MethodSection.content"

// The method section must explain every setting the model runs with, and nothing it no longer has. The chain:
// config/model.yaml is read with serde's deny_unknown_fields, every forecast records those settings and is validated
// against schemas/forecast.schema.json (which lists them exactly), and this test holds the section's labels to that
// list. A setting added to or dropped from the model therefore fails CI until this section says what it is.

interface ObjectSchema {
  properties: Record<string, ObjectSchema>
  additionalProperties?: boolean
}

const schema = JSON.parse(
  readFileSync(new URL("../../../../schemas/forecast.schema.json", import.meta.url), "utf8"),
) as {
  properties: { parameters: ObjectSchema }
}
const groups = schema.properties.parameters.properties

describe("method section", () => {
  it("labels exactly the settings a forecast records", () => {
    expect(Object.keys(groups).toSorted()).toEqual(Object.keys(method.content.parameters).toSorted())
    for (const [group, settings] of Object.entries(groups)) {
      expect(settings.additionalProperties, group).toBe(false)
      const labels = method.content.parameters[group as keyof typeof method.content.parameters]
      expect(Object.keys(settings.properties).toSorted(), group).toEqual(Object.keys(labels).toSorted())
    }
  })
})
