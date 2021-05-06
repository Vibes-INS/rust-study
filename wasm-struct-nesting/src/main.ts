// import { createApp } from 'vue'
// import App from './App.vue'
//
// createApp(App).mount('#app')

import init, { Counter, CounterChildren } from '@rsw/demo'

init().then(() => {
  const counter = new Counter()
  const children = counter.get_children()
  console.log(counter.add())
  console.log(counter.add())
  console.log(counter.add())
  console.log(children)

})
