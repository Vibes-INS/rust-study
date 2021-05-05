// import { createApp } from 'vue'
// import App from './App.vue'
//
// createApp(App).mount('#app')

import init, { Counter } from '@rsw/demo'

init().then(() => {
  const counter = Counter.new('10', 10)
})
