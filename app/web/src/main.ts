import "@fontsource-variable/bodoni-moda/opsz.css"
import "@fontsource-variable/bodoni-moda/opsz-italic.css"
import "@fontsource-variable/inter/opsz.css"
import { createApp } from "vue"
import { intlayer } from "vue-intlayer"
import App from "./App.vue"
import { initialLocale } from "./locales"
import "./style.css"

createApp(App).use(intlayer, { locale: initialLocale() }).mount("#app")
