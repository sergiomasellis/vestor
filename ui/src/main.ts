import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { sdInstall } from '@strizich/sdui'

const app = createApp(App)
sdInstall(app) // this is uggly we should chain it ! ;D

app.config.globalProperties.$filters = {
    toCurrency(value, cashflow=false) {
        if (typeof value !== "number") {
            return value;
        }

        var prepend = "";
        if(cashflow) {
            prepend = value > 0 ? "+" : ""; 
        }

        var formatter = new Intl.NumberFormat('en-US', {
            style: 'currency',
            currency: 'USD',
            minimumFractionDigits: 0
        });
        return prepend+formatter.format(value);
    }
}

app.use(router).mount('#app')
