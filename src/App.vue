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
  <div class="flex-col">
    <div class="max-w-sm mx-auto flex justify-center">
      <img class="h-20 " alt="Vue logo" src="./assets/logo.png" />
    </div>

    <div class="max-w-sm mx-auto bg-white  flex  items-center">
      <HelloWorld></HelloWorld>
    </div>
  </div>

</template>

<style>
  /* #app { */
  /* font-family: Avenir, Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale; */
  /* text-align: center; */
  /* color: #2c3e50; */
  /* margin-top: 60px; */
  /* } */
</style>