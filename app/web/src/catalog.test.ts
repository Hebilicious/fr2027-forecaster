import { readFileSync, readdirSync } from "node:fs"
import { join, relative } from "node:path"
import { parse } from "vue/compiler-sfc"
import { describe, expect, it } from "vitest"
import config from "../intlayer.config"

// Every word a visitor reads comes from a `*.content.ts` dictionary, and every dictionary entry exists in every
// locale. This test reads each component's template and fails on text written straight into it: a text node, or
// a static attribute a person reads or hears (title, alt, aria-label…), that contains a letter. It then reads
// every dictionary and fails on an entry that is empty in some locale or whose {{placeholders}} differ from the
// French, which is the reference.
//
// A template may still hold symbols, digits and punctuation ("·", "%", "→"), and anything bound with `:` or
// `{{ }}`, since those values come from a dictionary, the data or the formatters. Text that is the same in
// every language (a file path, a field name, statistical notation) goes in an element marked
// `translate="no"`, which this test skips, as do browsers' page translators.

const SRC = new URL(".", import.meta.url).pathname
const READ_ATTRIBUTES = new Set([
  "title",
  "alt",
  "placeholder",
  "aria-label",
  "aria-description",
  "aria-roledescription",
])
const LETTER = /\p{L}/u
// The parts of Vue's template AST this test reads. `NodeTypes` is a const enum Vue does not export at run time.
const ELEMENT = 1
const TEXT = 2
const ATTRIBUTE = 6
interface TemplateNode {
  type: number
  loc: { start: { line: number } }
  content?: unknown
  props?: { type: number; name: string; value?: { content: string }; loc: { start: { line: number } } }[]
  children?: TemplateNode[]
}

function vueFiles(dir: string): string[] {
  return readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
    const path = join(dir, entry.name)
    if (entry.isDirectory()) return vueFiles(path)
    return entry.name.endsWith(".vue") ? [path] : []
  })
}

/** Each piece of literal text in a template, as "line: text". */
function literalText(source: string): string[] {
  const ast = parse(source).descriptor.template?.ast as TemplateNode | undefined
  const found: string[] = []
  const visit = (node: TemplateNode) => {
    if (node.type === TEXT && typeof node.content === "string" && LETTER.test(node.content)) {
      found.push(`${node.loc.start.line}: ${node.content.trim()}`)
    }
    const props = node.type === ELEMENT ? (node.props ?? []) : []
    const attribute = (name: string) => props.find((prop) => prop.type === ATTRIBUTE && prop.name === name)
    if (attribute("translate")?.value?.content === "no") return
    for (const prop of props) {
      if (prop.type === ATTRIBUTE && READ_ATTRIBUTES.has(prop.name) && LETTER.test(prop.value?.content ?? "")) {
        found.push(`${prop.loc.start.line}: ${prop.name}="${prop.value?.content}"`)
      }
    }
    for (const child of node.children ?? []) visit(child)
  }
  if (ast) visit(ast)
  return found
}

describe("the UI catalog", () => {
  const files = vueFiles(SRC)

  it.each(files.map((file) => [relative(SRC, file), file]))("%s has no text outside a dictionary", (_name, file) => {
    expect(literalText(readFileSync(file, "utf8"))).toEqual([])
  })
})

const LOCALES = config.internationalization?.locales ?? []
const dictionaries = import.meta.glob<{ default: { key: string; content: unknown } }>("./**/*.content.ts", {
  eager: true,
})

interface Entry {
  path: string
  translation: Record<string, unknown>
}

/** Every translated entry in a dictionary's content, with its path. */
function entries(node: unknown, path: string): Entry[] {
  if (typeof node !== "object" || node === null) return []
  const record = node as Record<string, unknown>
  if (record["nodeType"] === "translation")
    return [{ path, translation: record["translation"] as Entry["translation"] }]
  if (record["nodeType"] === "insertion") return entries(record["insertion"], path)
  return Object.entries(record).flatMap(([name, child]) => entries(child, path ? `${path}.${name}` : name))
}

const placeholders = (text: string) => [...text.matchAll(/\{\{(\w+)\}\}/g)].map((m) => m[1]).toSorted()

describe("the dictionaries", () => {
  it("cover the locales in intlayer.config.ts", () => {
    expect(LOCALES).toContain("fr")
    expect(Object.keys(dictionaries).length).toBeGreaterThan(0)
  })

  it.each(Object.entries(dictionaries))("%s has every entry in every locale", (_file, module) => {
    const problems: string[] = []
    for (const { path, translation } of entries(module.default.content, "")) {
      const reference = translation["fr"]
      for (const locale of LOCALES) {
        const text = translation[locale]
        if (typeof text !== "string" || text.trim() === "") problems.push(`${path}: no ${locale} text`)
        else if (typeof reference === "string" && placeholders(text).join() !== placeholders(reference).join())
          problems.push(`${path}: ${locale} placeholders differ from fr`)
      }
    }
    expect(problems).toEqual([])
  })
})
