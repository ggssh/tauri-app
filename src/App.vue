<!-- <script setup>
  // This starter template is using Vue 3 <script setup> SFCs
  // Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
  import HelloWorld from './components/HelloWorld.vue'
</script> -->
<script>
  import HelloWorld from './components/HelloWorld.vue'
  export default {
    components: {
      HelloWorld
    },
  }
</script>

<template>
  <!-- <div class="p-6 max-w-sm mx-auto bg-white rounded-xl shadow-md flex  items-center space-x-4">
    <div class="flex-shrink-0">
      <img class="h-12 w-12" alt="Vue logo" src="./assets/logo.png" />
    </div>
    <div>
      <div class="text-xl font-medium text-black">ChitChat</div>
      <p class="text-gray-500">You have a new message!</p>
    </div>
    <HelloWorld></HelloWorld>
  </div> -->
  <div class="max-w-full mx-auto flex flex-col items-center">
    <img alt="Vue logo" src="./assets/logo.png" />

    <HelloWorld></HelloWorld>
  </div>
</template>

<style>
</style>