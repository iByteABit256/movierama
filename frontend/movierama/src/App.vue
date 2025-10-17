<template>
  <div id="app">
    <Navbar />
    <main class="content">
      <router-view />
    </main>
  </div>
</template>

<script setup>
import Navbar from './components/AppNavbar.vue'
</script>

<style>
body {
  margin: 0;
  font-family:
    system-ui,
    -apple-system,
    'Segoe UI',
    Roboto,
    sans-serif;
  background-color: #f8f8f8;
}

#app {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

.content {
  flex: 1;
  padding: 1.5rem;
}
</style>
