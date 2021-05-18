<template>
  <main>
    <router-view />
  </main>
</template>

<script lang="ts">
import { defineComponent } from 'vue'


export default defineComponent({
  name: 'App',
  components: {}
})
</script>

<style lang="scss">
  @use '../node_modules/@strizich/sdui/dist/scss/engine';
  @import url('../node_modules/@strizich/sdui/dist/style.css');
</style>