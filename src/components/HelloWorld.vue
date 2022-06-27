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
  import { open } from '@tauri-apps/api/dialog';
  // import { appDir } from '@tauri-apps/api/path';
  import { readTextFile, BaseDirectory } from '@tauri-apps/api/fs';

  export default {
    name: 'HelloWorld',
    data() {
      return {
        count: 0,
        msg: 'Hello Vue 3 + Vite',
        file_content: '',
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
        let msgs = [
          'Hello, World!',
          'Hello, Vue!',
          'Hello, Vue 3 + Vite!'
        ]
        invoke('greet', { name: 'World' })
          .then((response) => this.msg = msgs[this.count % 3] + response)
      },
      get_vec() {
        invoke('get_vec', { len: 5 })
          .then((response) => console.log(response))
      },
      get_map() {
        invoke('get_map', { len: 5 })
          .then((response) => console.log(response))
      },
      // 要用异步函数
      async open_file() {
        const selected = await open({
          multiple: false,
          defaultPath: 'F:\\main\\projects',
          filters: [
            {
              name: 'Text',
              extensions: ['txt']
            }]
        }).catch(err => console.log(err));
        if (Array.isArray(selected)) {
          // user selected multiple files
          console.log(selected);
          const contents = await readTextFile(selected[0]).catch(err => console.log(err));
          this.file_content = contents;
        } else if (selected === null) {
          // user cancelled the selection
          console.log('cancelled');
        } else {
          // user selected a single file
          // console.log(selected[0]);
          // for(let i = 0; i < selected.length; i++) {
          //   console.log(selected[i]);
          // }
          const contents = await readTextFile(selected).catch(err => console.log(err));
          this.file_content = contents;
        }
      }
    }
  }
</script>

<template>
  <div class="flex flex-col items-center">
    <h1 class="text-center">{{ msg }}</h1>
    <div class="flex">
      <button class="btn" type="button" @click="count++">count is: {{ count }}</button>
      <button class="btn" type="button" @click="my_invoke">invoke me</button>
      <button class="btn" type="button" @click="get_vec">get_vec</button>
      <button class="btn" type="button" @click="get_map">get_map</button>
      <button class="btn" type="button" @click="open_file">open_file</button>
    </div>
    <p class="text-center">{{file_content}}</p>
  </div>
</template>

<style scoped>
  .btn {
    @apply py-2 px-4 font-semibold rounded-lg shadow-md max-w-max ml-5 mr-5;
  }
</style>