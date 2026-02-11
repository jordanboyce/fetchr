import { createApp } from "vue";
import { createPinia } from "pinia";
import PrimeVue from "primevue/config";
import Aura from '@primevue/themes/aura';
import ToastService from "primevue/toastservice";
import Tooltip from "primevue/tooltip";
import "primeicons/primeicons.css";
import App from "./App.vue";

const app = createApp(App);
const pinia = createPinia();

app.use(pinia);
app.use(PrimeVue, {
  theme: {
    preset: Aura,
    options: {
      darkModeSelector: '.dark-mode',
      cssLayer: false
    }
  },
  ripple: true,
});
app.use(ToastService);
app.directive("tooltip", Tooltip);

app.mount("#app");
