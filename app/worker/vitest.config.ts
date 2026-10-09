import { defineConfig } from "vitest/config"

// Plain Node: the inbox logic is pure, and the SQL store runs against node:sqlite (whose
// "experimental" warning is silenced).
export default defineConfig({
  test: {
    include: ["src/**/*.test.ts"],
    environment: "node",
    execArgv: ["--disable-warning=ExperimentalWarning"],
  },
})
