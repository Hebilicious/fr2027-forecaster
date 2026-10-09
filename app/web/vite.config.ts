import vue from "@vitejs/plugin-vue"
import { defineConfig } from "vitest/config"

// The dev server proxies the API to `fr2027 serve`; FR2027_API picks another address.
const api = process.env["FR2027_API"] ?? "http://127.0.0.1:8027"

export default defineConfig({
  plugins: [vue()],
  server: {
    proxy: { "/api": api },
  },
  test: {
    include: ["src/**/*.test.ts"],
  },
})
