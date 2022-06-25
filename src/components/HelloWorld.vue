<!-- <script setup>
  import { ref } from 'vue'
  // now we can call our Command!
  // Right-click the application background and open the developer tools.
  // You will see "Hello, World!" printed in the console!
  invoke('greet', { name: 'World' })
    // `invoke` returns a Promise
    .then((response) => alert(response))
  defineProps({
    msg: String
  }),
  function 

  const count = ref(0)
</script> -->
<script>
  import { invoke } from "@tauri-apps/api"
  import { ref } from 'vue'

  export default {
    name: 'HelloWorld',
    data() {
      return {
        count: 0,
        msg: 'Hello Vue 3 + Vite'
      }
    },
    setup() {
      const count = ref(0)
      return {
        count
      }
    },
    methods: {
      my_invoke() {
        invoke('greet', { name: 'World' })
          .then((response) => this.msg = response)
      }
    }
  }
</script>

<template>
  <h1>{{ msg }}</h1>

  <p>
    Recommended IDE setup:
    <a href="https://code.visualstudio.com/" target="_blank">VS Code</a>
    +
    <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
  </p>

  <p>
    <a href="https://vitejs.dev/guide/features.html" target="_blank">
      Vite Documentation
    </a>
    |
    <a href="https://v3.vuejs.org/" target="_blank">Vue 3 Documentation</a>
  </p>

  <button type="button" @click="count++">count is: {{ count }}</button>
  <button type="button" @click="my_invoke">invoke</button>
  <p>
    Edit
    <code>components/HelloWorld.vue</code> to test hot module replacement.
  </p>
</template>

<style scoped>
  a {
    color: #42b983;
  }
</style>