// import { createApp } from 'vue'
// import App from './App.vue'
//
// createApp(App).mount('#app')

import init, { LinkedList } from '@rsw/linkedlist'

init().then(() => {
  const linkedList = LinkedList.new()

  linkedList.append(1)
  linkedList.append(2)
  linkedList.append(3)

  console.log(linkedList.test())
  // console.log(linkedList, linkedList.print(linkedList.pop()))
})
