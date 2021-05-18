import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { sdInstall } from '@strizich/sdui'

const app = createApp(App)
sdInstall(app) // this is uggly we should chain it ! ;D
app.use(router).mount('#app')
