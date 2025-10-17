<template>
  <div class="form-container">
    <h2>Login</h2>
    <form @submit.prevent="login">
      <div class="form-group">
        <label>Username</label>
        <input v-model="username" placeholder="Enter your username" required />
      </div>
      <div class="form-group">
        <label>Password</label>
        <input type="password" v-model="password" placeholder="Enter your password" required />
      </div>
      <button type="submit">Sign In</button>
    </form>
  </div>
</template>

<script setup>
import { ref } from 'vue'
const username = ref('')
const password = ref('')
function login() {
  alert(`Logging in as ${username.value}`)
}
</script>

<style scoped>
@import '../styles/form.css';
</style>
